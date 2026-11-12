# Agent guide

Read [README.md](README.md), [ARCHITECTURE.md](ARCHITECTURE.md), and [TESTING.md](TESTING.md). Follow current Dornglut Engineering standards and any accepted owning issue.

## Rules

- Keep generation semantics independent of applications and engine lifecycles.
- Preserve each framework's semantic ownership. No Runenwerk source transfer, framework dependency, plugin, or public API without accepted scope.
- Do not introduce speculative abstractions, shared runtimes, packages, or duplicate implementations.
- Preserve licensing and historical provenance in [LICENSING.md](LICENSING.md) and [BOOTSTRAP.md](BOOTSTRAP.md).
- Keep `cargo validate` repository-owned, CI read-only and pinned, and unsafe Rust forbidden.

## Delivery

1. Verify the accepted base, owning issue, relevant source, and concurrent writers.
2. Build one complete candidate off-ref; inspect the full authorized diff before publication.
3. Validate the exact feature head through repository CI (and locally when a suitable executor exists).
4. Critically review the complete change, resolve findings, and merge only under accepted repository controls.
