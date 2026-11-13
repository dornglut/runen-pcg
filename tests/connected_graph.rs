use std::collections::{BTreeSet, VecDeque};

use runen_pcg::{
    ConnectedGraph, ConnectedGraphAlgorithm, ConnectedGraphRequest, GenerationError, MAX_EDGES,
    MAX_NODES, Resource, generate_connected_graph,
};

fn request(
    node_count: u32,
    minimum_cycle_rank: u32,
    max_edges: u32,
    seed: u64,
) -> ConnectedGraphRequest {
    ConnectedGraphRequest {
        node_count,
        minimum_cycle_rank,
        max_edges,
        seed,
        algorithm: ConnectedGraphAlgorithm::SpanningTreeV1,
    }
}

fn endpoints(graph: &ConnectedGraph) -> Vec<(u32, u32)> {
    graph.edges().iter().map(|edge| edge.endpoints()).collect()
}

// Independent structural checks over the public crate API.
fn verify(graph: &ConnectedGraph, minimum_cycles: u32, max_edges: u32) {
    let n = graph.node_count() as usize;
    assert!(n >= 1);
    let edges = endpoints(graph);
    assert!(edges.len() <= max_edges as usize);
    let mut seen = BTreeSet::new();
    let mut adjacency = vec![Vec::new(); n];
    for &(left, right) in &edges {
        assert!(left < right && right < graph.node_count());
        assert!(seen.insert((left, right)));
        adjacency[left as usize].push(right as usize);
        adjacency[right as usize].push(left as usize);
    }
    assert!(edges.windows(2).all(|w| w[0] < w[1]));

    let mut reached = vec![false; n];
    let mut queue = VecDeque::from([0_usize]);
    reached[0] = true;
    while let Some(node) = queue.pop_front() {
        for &neighbor in &adjacency[node] {
            if !reached[neighbor] {
                reached[neighbor] = true;
                queue.push_back(neighbor);
            }
        }
    }
    assert!(reached.into_iter().all(|flag| flag));
    let cycles = edges.len() - (n - 1);
    let branches = adjacency
        .iter()
        .filter(|neighbors| neighbors.len() >= 3)
        .count();
    assert_eq!(cycles as u32, minimum_cycles);
    assert_eq!(graph.metrics().cycle_rank, cycles as u32);
    assert_eq!(graph.metrics().branching_vertices, branches as u32);
}

#[test]
fn explicit_version_and_single_node() {
    assert_eq!(
        ConnectedGraphAlgorithm::SpanningTreeV1.id(),
        "runen-pcg.graph.spanning-tree.v1"
    );
    let graph = generate_connected_graph(request(1, 0, 0, 0)).unwrap();
    assert_eq!(graph.node_count(), 1);
    assert_eq!(graph.edge_count(), 0);
    assert!(graph.edges().is_empty());
    verify(&graph, 0, 0);
}

#[test]
fn fixed_independent_golden_vectors() {
    let graph = generate_connected_graph(request(4, 1, 4, 0)).unwrap();
    assert_eq!(endpoints(&graph), [(0, 1), (1, 2), (1, 3), (2, 3)]);
    assert_eq!(graph.metrics().branching_vertices, 1);

    let graph = generate_connected_graph(request(6, 2, 12, 0x1234_5678_9abc_def0)).unwrap();
    assert_eq!(
        endpoints(&graph),
        [(0, 1), (0, 2), (0, 5), (1, 3), (2, 3), (2, 4), (4, 5)]
    );
    assert_eq!(graph.metrics().branching_vertices, 2);

    let graph = generate_connected_graph(request(8, 4, 16, 42)).unwrap();
    assert_eq!(
        endpoints(&graph),
        [
            (0, 1),
            (1, 3),
            (1, 5),
            (1, 6),
            (1, 7),
            (2, 4),
            (2, 6),
            (3, 4),
            (4, 5),
            (4, 7),
            (6, 7)
        ]
    );
    assert_eq!(graph.metrics().branching_vertices, 4);
}

#[test]
fn identical_seeds_replay_and_different_seeds_vary_edges() {
    let a = generate_connected_graph(request(6, 2, 12, 0x1234_5678_9abc_def0)).unwrap();
    let replay = generate_connected_graph(request(6, 2, 12, 0x1234_5678_9abc_def0)).unwrap();
    let b = generate_connected_graph(request(6, 2, 12, 0x1234_5678_9abc_def1)).unwrap();
    assert_eq!(a, replay);
    assert_ne!(a.edges(), b.edges());
    verify(&b, 2, 12);
}

#[test]
fn generated_candidates_obey_independent_invariants() {
    for n in 1..=24 {
        let possible = n * (n - 1) / 2 - (n - 1);
        for cycles in 0..=possible.min(8) {
            for seed in [0, 1, 42, u64::MAX] {
                let result = generate_connected_graph(request(n, cycles, 128, seed)).unwrap();
                verify(&result, cycles, 128);
            }
        }
    }
}

#[test]
fn invalid_infeasible_budget_and_resource_failures_are_typed() {
    assert_eq!(
        generate_connected_graph(request(0, 0, 0, 0)),
        Err(GenerationError::InvalidNodeCount)
    );
    assert_eq!(
        generate_connected_graph(request(3, 2, 10, 0)),
        Err(GenerationError::InfeasibleCycleRank {
            requested: 2,
            maximum: 1
        })
    );
    assert_eq!(
        generate_connected_graph(request(4, 1, 3, 0)),
        Err(GenerationError::InsufficientEdgeBudget {
            required: 4,
            limit: 3
        })
    );
    assert_eq!(
        generate_connected_graph(request(MAX_NODES + 1, 0, u32::MAX, 0)),
        Err(GenerationError::ResourceLimit {
            resource: Resource::Nodes,
            requested: u64::from(MAX_NODES) + 1,
            limit: u64::from(MAX_NODES)
        })
    );
    assert_eq!(
        generate_connected_graph(request(500, MAX_EDGES, u32::MAX, 0)),
        Err(GenerationError::ResourceLimit {
            resource: Resource::Edges,
            requested: u64::from(MAX_EDGES) + 499,
            limit: u64::from(MAX_EDGES)
        })
    );
    assert_eq!(
        generate_connected_graph(request(u32::MAX, u32::MAX, 0, 0)),
        Err(GenerationError::ResourceLimit {
            resource: Resource::Nodes,
            requested: u64::from(u32::MAX),
            limit: u64::from(MAX_NODES)
        })
    );
    assert_eq!(
        generate_connected_graph(request(2, 0, 0, 0)),
        Err(GenerationError::InsufficientEdgeBudget {
            required: 1,
            limit: 0
        })
    );
}

#[test]
fn upper_node_boundary_is_finite_without_quadratic_allocation() {
    let graph = generate_connected_graph(request(MAX_NODES, 0, MAX_EDGES, 9)).unwrap();
    assert_eq!(graph.edge_count(), MAX_NODES as usize - 1);
    verify(&graph, 0, MAX_EDGES);
}

#[test]
fn complete_small_graphs_remain_canonical_and_replayable() {
    for node_count in 2..=12 {
        let possible_edges = node_count * (node_count - 1) / 2;
        let cycle_rank = possible_edges - (node_count - 1);
        for seed in [0, 42, u64::MAX] {
            let graph =
                generate_connected_graph(request(node_count, cycle_rank, possible_edges, seed))
                    .unwrap();
            verify(&graph, cycle_rank, possible_edges);
            assert_eq!(
                graph.edge_count(),
                usize::try_from(possible_edges).unwrap()
            );
        }
    }
}

#[test]
fn combined_resource_boundaries_and_nearly_complete_graphs_are_valid() {
    let maximum_cycles = MAX_EDGES - (MAX_NODES - 1);
    let maximum = generate_connected_graph(request(
        MAX_NODES,
        maximum_cycles,
        MAX_EDGES,
        u64::MAX,
    ))
    .unwrap();
    assert_eq!(maximum.node_count(), MAX_NODES);
    assert_eq!(maximum.edge_count(), usize::try_from(MAX_EDGES).unwrap());
    verify(&maximum, maximum_cycles, MAX_EDGES);

    // The allowed output reaches the edge cap while nearly filling all
    // available node pairs. This exercises Floyd sampling near exhaustion.
    let node_count = 182;
    let total_pairs = node_count * (node_count - 1) / 2;
    assert!(total_pairs > MAX_EDGES);
    let cycle_rank = MAX_EDGES - (node_count - 1);
    let dense =
        generate_connected_graph(request(node_count, cycle_rank, MAX_EDGES, 42)).unwrap();
    assert_eq!(dense.edge_count(), usize::try_from(MAX_EDGES).unwrap());
    verify(&dense, cycle_rank, MAX_EDGES);
}
