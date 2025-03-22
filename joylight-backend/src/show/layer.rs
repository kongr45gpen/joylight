use std::sync::{Arc, RwLock};
use crate::fixture::{selection::{Selection, StrictSelection}, Fixture};

/// A blending mode, assigned to each layer, defines how multiple layers changing the same parameter are resolved
pub enum BlendingMode {
    /// Add this value to the current one
    Add,
    /// This layer takes precedence over the previous ones if it has a higher value.
    Highest,
    /// This layer takes precedence over the previous ones if it was updated recently.
    Latest,
    /// Multiply the current value with this one. This operation takes priority over all previous ones.
    Multiply,
    /// Override any previous value. This operation takes priority over all previous ones.
    Override,
    /// Multiply the current value with this one. This operation takes priority over previous multiplications
    /// and overrides.
    MultiplyOverride
}

struct FixtureParameterPair {
    pub fixture_uuid: uuid::Uuid,
    pub fixture: Arc<RwLock<Fixture>>,
    pub parameter: usize,
}

pub struct Layer {
    pub name: String,
    pub blending_mode: BlendingMode,
    pub priority: u32,
    pub active: bool,
    selection: StrictSelection,
}

impl Layer {
    pub fn new(name: &str, blending_mode: BlendingMode, priority: u32) -> Layer {
        Layer {
            name: name.to_string(),
            blending_mode,
            priority,
            active: true,
            selection: StrictSelection {
                name: format!("{} Selection", name),
                fixtures: vec![],
            }
        }
    }
}

