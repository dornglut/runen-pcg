# RunenPCG agent guide

Read README.md, ARCHITECTURE.md, TESTING.md, BOOTSTRAP.md and LICENSING.md before modifying this repository. Follow current Dornglut Engineering standards and the GPT Web GitHub procedure when applicable.

## Scope

This repository is bootstrapping an engine-neutral procedural-generation framework. There is no accepted procedural-generation public semantic API yet.

- Keep one coherent owner per semantic invariant and distinguish generator results from consumer acceptance.
- Do not copy or migrate Runenwerk's cave generator, create a Bevy/Runenwerk plugin, add a RunenGraph dependency, or invent a general generation runtime without a separately accepted issue.
- Do not introduce framework-wide generator/constraint trait systems, backend abstractions, speculative crate splits, or source mirrors.
- Do not make RunenPCG depend on Runenwerk, Chimera Signal, Runen Lab, Bevy, or other applications.
- Preserve GPL-3.0-only product representation, historical template provenance, and owner-only contribution policy until separately accepted changes.
- Keep `cargo validate` repository-owned; CI must remain an immutable pinned, read-only caller.
- Keep `unsafe_code = "forbid"` and the stable minimal toolchain. Do not claim an MSRV without evidence.

## Required workflow

1. Recheck current accepted main, owning issue, applicable Engineering authority and overlapping writers.
2. Read exact source and dependent public contracts before deciding implementation scope.
3. Prepare one complete candidate from an exact accepted base, with no branch-visible partial multi-file changes.
4. Validate through `cargo validate` using an available checked-out executor or exact-head hosted CI.
5. Critically review the entire accepted-base-to-feature-head diff; record ACCEPT or HOLD and reconcile findings.
6. Merge only the exact reviewed and validated head through the accepted repository controls. Consumer migrations and cross-repository adoption remain separate.

BOOTSTRAP.md is historical provenance, not ongoing template authority.
