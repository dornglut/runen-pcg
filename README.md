# RunenPCG

RunenPCG is Dornglut's proposed standalone, engine-neutral Rust framework for reusable procedural-generation capabilities. It is not a Bevy plugin or a Runenwerk subsystem.

## Maturity

**Bootstrap only.** The repository currently defines no accepted procedural-generation algorithms, constraints, candidate format, public semantic API, or compatibility promise. A crate identity and validation baseline do not establish consumer readiness.

The initial independent use cases to investigate are the existing Runenwerk cave-topology generator and Chimera Signal's procedural Site work. Neither source transfer nor consumer adoption is authorized by this bootstrap.

## Ownership boundary

RunenPCG is intended to own reusable procedural-generation methods and their algorithm-specific input, determinism, failure, and result contracts when independently justified.

It does **not** own graph relationship semantics (RunenGraph), spatial addressing and availability (RunenSpatial), signed-field mathematics (RunenSDF), Runenwerk authored source/product ratification, Chimera Site/gameplay meaning, engine lifecycle, asset persistence, geometry/collision activation, or cross-framework orchestration.

Dependencies on peer frameworks are introduced by proven capability-specific needs; none are required for this bootstrap. No universal generator/constraint trait, execution service, or port-and-adapter runtime is implied.

Cross-repository Runen-family recognition is separately owned by Dornglut Engineering and is not conferred by a bootstrap file.

## Repository authority

- [Architecture](ARCHITECTURE.md) — local boundary and dependency rules.
- [Agent instructions](AGENTS.md) — executor constraints.
- [Testing](TESTING.md) — canonical merge-readiness checks.
- [Bootstrap provenance](BOOTSTRAP.md) — template origin and historical licensing.
- [Licensing](LICENSING.md) — public and separate possible commercial paths.
- [Dornglut Engineering](https://github.com/dornglut/engineering/blob/main/architecture/runen-family.md) — cross-repository framework membership.

## Validation

```text
cargo validate
```

## License

Current RunenPCG source is available under the [GNU General Public License v3.0-only](LICENSE). [Licensing](LICENSING.md) describes the separate possible commercial path and the historical Apache-2.0 template grant.
