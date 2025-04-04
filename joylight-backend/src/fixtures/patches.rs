//! Patches describe how fixtures are linked to their output.

use std::{any::Any, fmt::Debug};

/// A patch describes how and where a fixture should be represented in its output encoding.
/// It can be a DMX universe, an MQTT path, or anything else that explains where we should
/// send this fixture's parameters.
pub trait Patch: Any + Debug {

}

/// A patch that leads to nowhere. Will probably show an error if the fixture is asked to
/// output anything. Useful for demos and examples.
#[derive(Debug)]
pub struct NullPatch {
}

impl Patch for NullPatch {}

impl Default for NullPatch {
    fn default() -> Self {
        Self {}
    }
}

/// A patch for a DMX universe.
///
/// If a parameter's address overflows a universe, it will continue to the next universe.
#[derive(Clone, Debug)]
pub struct DMXPatch {
    /// DMX universe, starting from 0
    pub universe: usize,
    /// DMX start address, 0-511
    pub start_address: u16,
}

impl DMXPatch {
    pub fn new(universe: usize, start_address: u16) -> Self {
        Self {
            universe,
            start_address,
        }
    }

    /// Special duck function.
    pub fn duck(&self) {
        println!("I AM A DUCK !! QUACK QUACK");
    }
}

impl Patch for DMXPatch {}
