# RunenPCG

RunenPCG is an engine-independent Rust library for reusable procedural generation.

## Status

**Bootstrap only.** No generation algorithms or public semantic API are implemented yet. The first capability under investigation is deterministic graph generation for independent consumers such as Runenwerk and Chimera Signal.

## Scope

RunenPCG is intended to own generation algorithms and their input, determinism, failure, and result contracts. It may compose RunenGraph, RunenSpatial, or RunenSDF when a capability requires them.

Applications retain authoring rules, gameplay meaning, world admission, geometry, scheduling, persistence, and runtime integration. RunenPCG is not an engine plugin.

See [Architecture](ARCHITECTURE.md) for ownership and dependency rules.

## Development

```text
cargo validate
```

See [Testing](TESTING.md), [Agent guide](AGENTS.md), and [Bootstrap provenance](BOOTSTRAP.md). Organization policies: [Contributing](https://github.com/dornglut/.github/blob/main/CONTRIBUTING.md) and [Security](https://github.com/dornglut/.github/blob/main/SECURITY.md).

## License

[GPL-3.0-only](LICENSE). See [Licensing](LICENSING.md) for historical template rights and possible separate commercial terms.
