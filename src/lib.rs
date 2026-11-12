//! Engine-independent procedural-generation library.
//!
//! The first capability produces one deterministic, bounded, simple undirected
//! graph candidate. Consumers retain all application and runtime authority.

mod connected;

pub use connected::{
    ConnectedGraph, ConnectedGraphAlgorithm, ConnectedGraphRequest, Edge, GenerationError,
    GraphMetrics, MAX_EDGES, MAX_NODES, Resource, generate_connected_graph,
};
