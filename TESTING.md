# Testing

## Canonical validation

```text
cargo validate
```

The local `xtask` checks repository identity/licensing, Rust formatting,
locked tests, Clippy, rustdoc, Git whitespace and clean worktree preservation.
The pinned GitHub workflow executes the command on the exact reviewed head.

## Public conformance

`tests/connected_graph.rs` uses the standalone public API and covers:

- pinned algorithm ID and independent golden replay vectors;
- connectivity, simple canonical edges, cycle rank and measured branching;
- seed reproducibility and representative variation;
- zero and single-node behavior, infeasible cycles, edge budgets and finite
  node/edge resource limits.

These tests prove the first graph-formation capability, not application
ratification, historical Runenwerk replay, spatial geometry, runtime integration,
or arbitrary-scale performance. A changed feature head requires new CI evidence
and critical review.
