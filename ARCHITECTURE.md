# RunenPCG architecture

## Framework boundary

RunenPCG is a proposed standalone semantic owner for reusable, engine-independent procedural-generation capabilities, *not* a general framework for graph ownership, 3D terrain, world state, game content, job scheduling, or engine plugins.

The first concrete capability and public contract require separate issue-owned design, conformance evidence, and consumer review. The initial candidate is generic graph-based generation, informed by Runenwerk's existing deterministic cave-topology behavior and Chimera Signal's independent procedural Site needs. Similarity does not authorize copying existing source.

## Responsibility and authority

Reusable generation algorithms may define their own typed requests, deterministic/versioned algorithm behavior, bounded evaluation, observable errors, and verifiable results. Such semantics must be accepted per capability; this bootstrap defines none of them.

RunenGraph retains its graph/relationship semantics. RunenSpatial retains spatial-addressing semantics. RunenSDF retains signed-field mathematics. Runenwerk retains authored source, cave/document ratification, product admission/publication, engine execution, and integration. Chimera retains Site/gameplay semantics, spatial realization, physics, and server/client authority.

A generator result is not automatically an accepted game world, a spatial reservation, or a published runtime product.

## Dependency direction

```text
runen-graph / runen-spatial / runen-sdf   (only when separately justified)
                     |
                     v
         runen-pcg capability
                     |
          +----------+----------+
          v                     v
   Runenwerk adapters     Chimera integration
          |
          v
      Runen Lab applications (when relevant)
```

This is an intended direction, not a claim of current dependencies or accepted cross-repository adoption. The base library must remain independently usable without Bevy, Runenwerk or Chimera dependencies. Engine-specific plugins may be downstream integrations only when concrete pressure warrants them.

## Physical repository shape

One product crate (`runen-pcg`) plus the repository-local `xtask`. There is no product implementation yet. Do not pre-create packages, modules, external dependencies, shared adapters, or architecture taxonomies without a concrete accepted need.

## Evolution and transfer

Public compatibility, seed replay, algorithm versions and deterministic scope must be defined before a real generator contract is accepted. Future extraction from Runenwerk must meet Dornglut Engineering's single-authority transfer and consumer cutover gates; there must not be two independently writable copies of the same generator.

Repository-local authority does not itself add RunenPCG to the accepted Runen-family registry. That remains an Engineering decision.
