# Connected undirected graph generation v1

This document fixes the replay and output contract of
`ConnectedGraphAlgorithm::SpanningTreeV1`
(`runen-pcg.graph.spanning-tree.v1`). Future changes that alter any returned
edge for a valid request require a **new algorithm identifier**.

## Input and outcomes

`ConnectedGraphRequest` contains a positive `node_count`, a
`minimum_cycle_rank`, `max_edges`, a numeric `seed`, and an explicit algorithm.

The generator produces **one** candidate with nodes `0..node_count`. It
constructs exactly `node_count - 1 + minimum_cycle_rank` undirected edges,
or returns a typed error. These edges form a connected, simple graph.

For connected graphs, cycle rank is `E - V + 1`. It is not a count of
edge-disjoint routes or evidence of physically traversable corridors.
`branching_vertices` counts nodes of degree at least three. Both metrics
are measured from the canonical output, never accepted as caller claims.

A single node with zero edges and zero cycles is valid. Zero nodes is invalid.
A mathematically impossible cycle count is infeasible. A caller edge budget
below the required count is insufficient. Work exceeding the first version's
resource limits is rejected. A failed attempt never returns partial output.

## Resource contract

- Maximum nodes: **4096**.
- Maximum generated edges: **16384**.
- Algorithm uses `O(V + E)` memory, including candidate output, spanning
  tree, sampled additions and structural verification.
- The version uses `O(V log V + E log E + E log² V)` time as a conservative
  upper characterization. It never builds the complete set of unused pairs.
- All random choices are finite; feasible, admitted requests do not exhaust
  a hidden attempt budget. Allocation remains bounded by the explicit caps.

These are initial finite limits, not claims of arbitrary-scale support.

## Exact replay law

1. Initialize a 64-bit SplitMix64 state to `seed`. Each draw increments state
   by `0x9e3779b97f4a7c15` with wrapping arithmetic, then applies the standard
   two xor-shift/multiply mixing steps (constants
   `0xbf58476d1ce4e5b9` and `0x94d049bb133111eb`), then xor-shift 31.
2. Map each draw to `[0, bound)` using unsigned 64-bit remainder. Bounds are
   always positive. This deterministic modulo operation is not guaranteed to
   produce mathematically uniform bounded samples.
3. Start with `[0, 1, ..., node_count-1]`; perform descending-index
   Fisher–Yates swaps for indices `node_count-1` down to `1` (one draw with
   bound `index+1` per swap).
4. For ascending permutation indices `1..node_count`, draw one parent index
   in `[0,index)`. Form the canonical edge from each node to the selected
   earlier permutation node. This creates a spanning tree.
5. Assign every possible pair `(a,b)`, `a < b`, a rank by increasing
   `a`, then increasing `b`: `a*(2*n-a-1)/2 + (b-a-1)`.
   Exclude the `n-1` tree-pair ranks.
6. Let `available` be the remaining pair count and `k` the requested
   cycle rank. Use Floyd sampling of `k` unique **eligible** indices:
   for ascending `cursor` in `available-k..available`, draw
   `pick = next() % (cursor+1)`; insert `pick` if absent, otherwise insert
   `cursor`.
7. Map sampled eligible indices, in ascending order, to full pair ranks by
   skipping excluded tree ranks; unrank into canonical pairs. Sort all edges
   lexicographically.
8. Independently verify endpoint bounds, strict ordering, uniqueness,
   connectedness and minimum cycle rank. Return metrics derived from verified
   edges.

The number of RNG draws is exactly `2*(V-1) + k` for this version.
The fixed vectors in `tests/connected_graph.rs` pin replay semantics.

## Ownership and compatibility

The returned graph is an immutable candidate, not a graph-storage framework,
consumer source identity, accepted game product, or spatial proof. RunenGraph
may be used by separately justified consumers, but it is not a dependency.

Runenwerk's `procgen.cave.generate.v1` has a **different** source-derived
seed and selection procedure. This version does not claim to replay it.
Runenwerk migration requires a separately authorized and compile-proven
compatibility cutover with historical fixtures and old-source retirement.
