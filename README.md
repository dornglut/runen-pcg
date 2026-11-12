# RunenPCG

RunenPCG is an engine-independent Rust library for reusable procedural generation.

## Status

The first capability generates a bounded, deterministic connected undirected
graph from explicit inputs. No engine runtime, graph backend, scheduler, or
application adapter is required.

## Usage

```rust
use runen_pcg::{
    ConnectedGraphAlgorithm, ConnectedGraphRequest, generate_connected_graph,
};

let graph = generate_connected_graph(ConnectedGraphRequest {
    node_count: 12,
    minimum_cycle_rank: 2,
    max_edges: 24,
    seed: 42,
    algorithm: ConnectedGraphAlgorithm::SpanningTreeV1,
})?;

assert_eq!(graph.node_count(), 12);
assert_eq!(graph.metrics().cycle_rank, 2);
# Ok::<(), runen_pcg::GenerationError>(())
```

The output contains canonical zero-based endpoints. The application assigns
domain identities, selects or ratifies candidates, and realizes geometry.

The [v1 contract](spec/connected-graph-v1.md) documents the replay law,
resource limits and failure semantics. [Architecture](ARCHITECTURE.md)
documents the framework boundary.

## Development

```text
cargo validate
```

See [Testing](TESTING.md) and [Agent guide](AGENTS.md).
Organization policies: [Contributing](https://github.com/dornglut/.github/blob/main/CONTRIBUTING.md)
and [Security](https://github.com/dornglut/.github/blob/main/SECURITY.md).

## License

[GPL-3.0-only](LICENSE). See [Licensing](LICENSING.md) for historical
template rights and possible separate commercial terms.
