use std::{collections::BTreeSet, error::Error, fmt};

/// Maximum admitted nodes for the first versioned graph generator.
pub const MAX_NODES: u32 = 4_096;
/// Maximum generated edges for the first versioned graph generator.
pub const MAX_EDGES: u32 = 16_384;

/// An explicitly selected, replay-stable graph generation algorithm.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConnectedGraphAlgorithm {
    /// A randomized spanning tree with a sampled set of additional edges.
    SpanningTreeV1,
}

impl ConnectedGraphAlgorithm {
    /// The versioned algorithm identifier recorded by callers for replay.
    pub const fn id(self) -> &'static str {
        match self {
            Self::SpanningTreeV1 => "runen-pcg.graph.spanning-tree.v1",
        }
    }
}

/// A single bounded generation attempt. Node keys are contiguous from zero.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ConnectedGraphRequest {
    /// Number of graph nodes; at least one.
    pub node_count: u32,
    /// Minimum independent cycle rank; V1 generates exactly this many cycles.
    pub minimum_cycle_rank: u32,
    /// Edge-count ceiling, not a target; V1 emits `node_count - 1 + minimum_cycle_rank`.
    pub max_edges: u32,
    /// Raw numeric seed; never implicitly sourced from the host.
    pub seed: u64,
    /// Explicit versioned algorithm choice.
    pub algorithm: ConnectedGraphAlgorithm,
}

/// An undirected, canonical, non-self edge. Endpoints are zero-based indices.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Edge {
    first: u32,
    second: u32,
}

impl Edge {
    fn canonical(a: u32, b: u32) -> Self {
        if a < b {
            Self {
                first: a,
                second: b,
            }
        } else {
            Self {
                first: b,
                second: a,
            }
        }
    }

    /// Canonical endpoints, with first strictly less than second.
    pub const fn endpoints(self) -> (u32, u32) {
        (self.first, self.second)
    }
}

/// Structural facts recomputed from the returned edges.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GraphMetrics {
    /// Cycle rank E - V + 1 for this connected graph.
    pub cycle_rank: u32,
    /// Number of vertices of degree at least three.
    pub branching_vertices: u32,
}

/// An immutable generated connected simple undirected graph.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConnectedGraph {
    node_count: u32,
    edges: Vec<Edge>,
    metrics: GraphMetrics,
}

impl ConnectedGraph {
    /// Number of nodes, including isolated single-node graphs.
    pub const fn node_count(&self) -> u32 {
        self.node_count
    }

    /// Number of unique undirected edges.
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    /// Canonically sorted, unique edges.
    pub fn edges(&self) -> &[Edge] {
        &self.edges
    }

    /// Independently measured structural facts.
    pub const fn metrics(&self) -> GraphMetrics {
        self.metrics
    }
}

/// The resource constrained by a generation limit.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Resource {
    Nodes,
    Edges,
}

/// Typed, deterministic failures for a single attempt.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GenerationError {
    InvalidNodeCount,
    InfeasibleCycleRank {
        requested: u32,
        maximum: u64,
    },
    InsufficientEdgeBudget {
        required: u64,
        limit: u32,
    },
    ResourceLimit {
        resource: Resource,
        requested: u64,
        limit: u64,
    },
    InternalInvariant,
}

impl fmt::Display for GenerationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidNodeCount => write!(f, "node count must be positive"),
            Self::InfeasibleCycleRank { requested, maximum } => {
                write!(
                    f,
                    "cycle rank {requested} exceeds feasible maximum {maximum}"
                )
            }
            Self::InsufficientEdgeBudget { required, limit } => {
                write!(f, "{required} edges required, but the budget is {limit}")
            }
            Self::ResourceLimit {
                resource,
                requested,
                limit,
            } => write!(
                f,
                "{resource:?} resource request {requested} exceeds limit {limit}"
            ),
            Self::InternalInvariant => write!(f, "generated graph violates structural invariants"),
        }
    }
}

impl Error for GenerationError {}

/// Generate exactly one connected simple-undirected-graph candidate.
///
/// This is pure and reproducible for the selected algorithm version. It does
/// not schedule work, ratify a consumer product, or realize physical geometry.
pub fn generate_connected_graph(
    request: ConnectedGraphRequest,
) -> Result<ConnectedGraph, GenerationError> {
    let n = u64::from(request.node_count);
    if n == 0 {
        return Err(GenerationError::InvalidNodeCount);
    }
    if request.node_count > MAX_NODES {
        return Err(GenerationError::ResourceLimit {
            resource: Resource::Nodes,
            requested: n,
            limit: u64::from(MAX_NODES),
        });
    }

    let tree_edges = n - 1;
    let possible_edges = n * (n - 1) / 2;
    let possible_cycles = possible_edges - tree_edges;
    if u64::from(request.minimum_cycle_rank) > possible_cycles {
        return Err(GenerationError::InfeasibleCycleRank {
            requested: request.minimum_cycle_rank,
            maximum: possible_cycles,
        });
    }
    let required_edges = tree_edges + u64::from(request.minimum_cycle_rank);
    if required_edges > u64::from(request.max_edges) {
        return Err(GenerationError::InsufficientEdgeBudget {
            required: required_edges,
            limit: request.max_edges,
        });
    }
    if required_edges > u64::from(MAX_EDGES) {
        return Err(GenerationError::ResourceLimit {
            resource: Resource::Edges,
            requested: required_edges,
            limit: u64::from(MAX_EDGES),
        });
    }

    let edges = match request.algorithm {
        ConnectedGraphAlgorithm::SpanningTreeV1 => {
            spanning_tree_v1(request.node_count, request.minimum_cycle_rank, request.seed)
        }
    };
    let metrics = measure_and_verify(request.node_count, &edges, request.minimum_cycle_rank)?;
    Ok(ConnectedGraph {
        node_count: request.node_count,
        edges,
        metrics,
    })
}

fn spanning_tree_v1(node_count: u32, extra_edges: u32, seed: u64) -> Vec<Edge> {
    let n = usize::try_from(node_count).expect("node count within library bound");
    let mut rng = SplitMix64(seed);
    let mut nodes: Vec<u32> = (0..node_count).collect();
    for index in (1..nodes.len()).rev() {
        let other = usize::try_from(rng.index((index + 1) as u64))
            .expect("random index within library bound");
        nodes.swap(index, other);
    }

    let mut edges = Vec::with_capacity(n - 1 + extra_edges as usize);
    for index in 1..n {
        let parent =
            usize::try_from(rng.index(index as u64)).expect("parent index within library bound");
        edges.push(Edge::canonical(nodes[index], nodes[parent]));
    }

    // Sample from pair indices *excluding* the spanning-tree edges. Unlike
    // enumerating every unused pair, this uses O(V + E) memory.
    let mut excluded: Vec<u64> = edges
        .iter()
        .map(|edge| pair_rank(edge.first, edge.second, node_count))
        .collect();
    excluded.sort_unstable();
    let possible_pairs = u64::from(node_count) * u64::from(node_count - 1) / 2;
    let available = possible_pairs - (n - 1) as u64;
    let extra = u64::from(extra_edges);
    let mut selected = BTreeSet::<u64>::new();
    for cursor in (available - extra)..available {
        let picked = rng.index(cursor + 1);
        if !selected.insert(picked) {
            selected.insert(cursor);
        }
    }

    for index in selected {
        let rank = skip_excluded(index, &excluded);
        edges.push(pair_from_rank(rank, node_count));
    }
    edges.sort_unstable();
    edges
}

fn pair_rank(a: u32, b: u32, node_count: u32) -> u64 {
    row_start(u64::from(a), u64::from(node_count)) + u64::from(b - a - 1)
}

fn row_start(row: u64, node_count: u64) -> u64 {
    row * (2 * node_count - row - 1) / 2
}

/// Map an ordinal in the complement of excluded pair ranks to a full pair rank.
fn skip_excluded(index: u64, excluded: &[u64]) -> u64 {
    let mut low = index;
    let mut high = index + excluded.len() as u64;
    while low < high {
        let middle = low + (high - low) / 2;
        let skipped = excluded.partition_point(|&rank| rank <= middle) as u64;
        if middle + 1 - skipped > index {
            high = middle;
        } else {
            low = middle + 1;
        }
    }
    low
}

fn pair_from_rank(rank: u64, node_count: u32) -> Edge {
    let n = u64::from(node_count);
    let mut low = 0_u64;
    let mut high = n - 1;
    while low + 1 < high {
        let middle = low + (high - low) / 2;
        if row_start(middle, n) <= rank {
            low = middle;
        } else {
            high = middle;
        }
    }
    let right = low + 1 + rank - row_start(low, n);
    Edge {
        first: u32::try_from(low).expect("node rank within library bound"),
        second: u32::try_from(right).expect("node rank within library bound"),
    }
}

fn measure_and_verify(
    node_count: u32,
    edges: &[Edge],
    minimum_cycles: u32,
) -> Result<GraphMetrics, GenerationError> {
    let n = usize::try_from(node_count).map_err(|_| GenerationError::InternalInvariant)?;
    if n == 0 || edges.len() < n - 1 {
        return Err(GenerationError::InternalInvariant);
    }
    let mut adjacency = vec![Vec::<u32>::new(); n];
    let mut previous: Option<Edge> = None;
    for &edge in edges {
        if edge.first >= edge.second
            || edge.second >= node_count
            || previous.is_some_and(|prior| prior >= edge)
        {
            return Err(GenerationError::InternalInvariant);
        }
        adjacency[edge.first as usize].push(edge.second);
        adjacency[edge.second as usize].push(edge.first);
        previous = Some(edge);
    }

    let mut reached = vec![false; n];
    let mut pending = vec![0_u32];
    reached[0] = true;
    let mut count = 1_usize;
    while let Some(node) = pending.pop() {
        for &neighbor in &adjacency[node as usize] {
            if !reached[neighbor as usize] {
                reached[neighbor as usize] = true;
                count += 1;
                pending.push(neighbor);
            }
        }
    }
    if count != n {
        return Err(GenerationError::InternalInvariant);
    }

    let cycle_rank =
        u32::try_from(edges.len() - (n - 1)).map_err(|_| GenerationError::InternalInvariant)?;
    if cycle_rank < minimum_cycles {
        return Err(GenerationError::InternalInvariant);
    }
    let branching_vertices = u32::try_from(
        adjacency
            .iter()
            .filter(|neighbors| neighbors.len() >= 3)
            .count(),
    )
    .map_err(|_| GenerationError::InternalInvariant)?;
    Ok(GraphMetrics {
        cycle_rank,
        branching_vertices,
    })
}

/// Explicit SplitMix64 state; changing draw order changes algorithm identity.
struct SplitMix64(u64);

impl SplitMix64 {
    fn index(&mut self, bound: u64) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut value = self.0;
        value = (value ^ (value >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        value = (value ^ (value >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        (value ^ (value >> 31)) % bound
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pair_rank_and_unrank_are_inverses_over_small_graphs() {
        for node_count in 2_u32..=100 {
            let mut expected_rank = 0_u64;
            for first in 0..node_count {
                for second in (first + 1)..node_count {
                    assert_eq!(pair_rank(first, second, node_count), expected_rank);
                    assert_eq!(
                        pair_from_rank(expected_rank, node_count).endpoints(),
                        (first, second)
                    );
                    expected_rank += 1;
                }
            }
            assert_eq!(
                expected_rank,
                u64::from(node_count) * u64::from(node_count - 1) / 2
            );
        }
    }

    #[test]
    fn excluded_pair_indices_map_to_every_remaining_pair() {
        for node_count in 2_u32..=5 {
            let pair_count = u64::from(node_count) * u64::from(node_count - 1) / 2;
            for mask in 0_u64..(1_u64 << pair_count) {
                let excluded: Vec<u64> = (0..pair_count)
                    .filter(|&rank| mask & (1_u64 << rank) != 0)
                    .collect();
                let available = (0..pair_count)
                    .filter(|&rank| mask & (1_u64 << rank) == 0);
                for (index, expected_rank) in available.enumerate() {
                    let index = u64::try_from(index).expect("bounded small-graph index");
                    assert_eq!(skip_excluded(index, &excluded), expected_rank);
                }
            }
        }
    }
}
