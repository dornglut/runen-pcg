# Architecture

## Responsibility

RunenPCG is intended to provide reusable, engine-independent generation algorithms and their typed inputs, deterministic/versioned behavior, bounded execution, diagnostics, and results.

Capabilities receive their own public contracts when justified and accepted. The bootstrap defines no generation API.

## Ownership boundaries

| Owner | Responsibility |
| --- | --- |
| RunenPCG | Reusable procedural-generation algorithms and their contracts |
| RunenGraph | Graph identities, relationships, and graph analysis |
| RunenSpatial | Spatial addressing and availability semantics |
| RunenSDF | Signed-field mathematics |
| Runenwerk | Authored sources, candidate/product admission, engine integration |
| Chimera Signal | Site/gameplay semantics, 3D realization, physics, server/client authority |

Generation produces candidate data; it does not authorize a world change or publish a runtime product.

## Dependencies

```text
RunenGraph / RunenSpatial / RunenSDF (as justified)
                    |
                    v
                 RunenPCG
                    |
                    v
       Runenwerk / Chimera / Runen-Lab
```

Peer-framework dependencies are optional and capability-specific. RunenPCG must not depend on its applications, Bevy, or Runenwerk's engine. Engine plugins and adapters remain consumer-owned.

## Evolution

The initial repository has one library crate and a local `xtask`. Do not add generic trait systems, plugin infrastructure, or crate splits without concrete evidence.

Runenwerk's existing cave-topology generator motivates the first investigation, but its source remains Runenwerk-owned until an accepted extraction. A transfer must preserve replay compatibility, qualify independent consumers, and retire duplicate implementations.

Dornglut Engineering owns cross-repository framework recognition; this document establishes only the local boundary.
