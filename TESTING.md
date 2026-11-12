# RunenPCG testing and validation

## Canonical command

```text
cargo validate
```

The repository-local `xtask` is the sole current validation authority. The immutable pinned shared CI workflow only invokes it.

## Current baseline

Validation checks required repository files, RunenPCG and GPL-3.0-only identity, absence of active template identity, Rust formatting, locked workspace tests, Clippy with warnings denied, rustdoc with warnings denied, Git whitespace and clean worktree preservation.

No generation capability, public conformance suite, historical seed replay, or independent consumer workload is implemented at bootstrap. Do not interpret the baseline as such proof.

## Acceptance evidence

Repository-owned hosted CI must execute against the exact reviewed pull-request feature head. Changing that head invalidates previous check and review evidence. A future public procedural-generation contract requires behavior-focused, seed/version stability, resource-bound and independent downstream conformance evidence in addition to this baseline.

`cargo validate` must remain runnable without repository-writing credentials. Repo settings must require the canonical check before merge; CI success alone does not grant merge authorization.
