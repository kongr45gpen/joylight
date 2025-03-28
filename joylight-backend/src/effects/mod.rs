//! Joylight's effects engine is a node-based engine.
//!
//! The node graph is synchronous, meaning that one input should create timely outputs in the same cycle.
//!
//! Inputs, intermediate values and outputs can be of different types, or even contain multiple values in a vector.
//!
//! Node values are based on view values, as they can be easily converted to each other and allow users to work on different representations
//! based on their needs (high-level or low-level).

pub mod graph;
pub mod io;
pub mod node;
pub mod nodes;
