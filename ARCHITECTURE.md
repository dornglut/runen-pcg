# Architecture

## Responsibility

RunenPCG owns reusable, engine-independent generation algorithms and their
capability-specific input, determinism, limits, errors, and output contracts.

The first capability generates a connected simple undirected graph in one
seeded attempt. Its canonical edges are immutable output data, not a second
graph-storage authority. See [the v1 contract](spec/connected-graph-v1.md).

## Ownership boundaries

| Owner | Responsibility |
| --- | --- |
| RunenPCG | Reusable generation algorithms and contracts |
| RunenGraph | Graph identities, relationships, and general graph analysis |
| RunenSpatial | Spatial addressing and availability semantics |
| RunenSDF | Signed-field mathematics |
| Runenwerk | Authored sources, candidate/product admission, engine integration |
| Chimera Signal | Site/gameplay semantics, 3D realization, physics, server/client authority |

Generation produces candidate data; it does not authorize a world change or
publish a runtime product.

## Dependencies

```text
RunenGraph / RunenSpatial / RunenSDF (only as justified)
                     |
                     v
                  RunenPCG
                     |
                     v
        Runenwerk / Chimera / Runen-Lab
```

The initial graph generator has no peer or runtime dependencies. Engine plugins
and adapters remain consumer-owned.

## Evolution

Keep one product crate and the local `xtask` unless a proven boundary requires
a split. Do not add a universal generation trait, storage framework, scheduler,
or generic constraints DSL without concrete independent use.

Runenwerk's existing cave-topology algorithm remains Runenwerk-owned. A future
transfer must preserve historical replay under its existing identity, qualify
consumers, and remove duplicate implementations. [Engineering ADR 0012](https://github.com/dornglut/engineering/blob/main/adrs/0012-establish-runen-pcg-boundary.md)
establishes the cross-repository authority; it does not authorize cutover.
