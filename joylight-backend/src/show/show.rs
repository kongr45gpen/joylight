use std::collections::HashMap;
use std::error::Error;
use std::sync::{Arc, RwLock, Weak};

use anyhow::Context;
use log::*;

use super::layer::*;
use crate::fixtures::selection::Selection;
use crate::fixtures::FixtureRef;
use crate::parameters::parameter_value::ParameterValue;
use crate::utils::SmartRef;

#[derive(Debug, Default)]
pub struct Show {
    /// The fixtures that are currently in the show
    pub fixtures: HashMap<String, FixtureRef>,
    /// Weak pointers to active selections that should be updated when necessary
    selections: Vec<Weak<RwLock<dyn Selection>>>,
    layers: Vec<SmartRef<Layer>>,
}

impl Show {
    pub fn add_fixture(&mut self, fixture: FixtureRef) {
        let name = fixture.read(|f| f.name.clone()).unwrap_or_else(|_| "Unnamed".into());
        self.fixtures.insert(name, fixture);
    }

    pub fn add_selection(&mut self, selection: Arc<RwLock<dyn Selection>>) {
        self.selections.push(Arc::downgrade(&selection));
    }

    /// Callback to refresh fixtures in selections
    pub fn refresh_fixtures(&mut self) {
        for selection in self.selections.iter().filter_map(|weak| weak.upgrade()) {
            let guard = selection.write();
            if let Ok(mut selection) = guard {
                selection.update(self);
            } else if let Err(e) = guard {
                error!("Failed to lock selection for update: {}", e);
            }
        }

        // Remove dangling selections (garbage collect...)
        // This may not be the fastest way but it is fully safe
        self.selections.retain(|weak| weak.strong_count() > 0);
    }

    pub fn add_layer(&mut self, layer: SmartRef<Layer>) {
        self.layers.push(layer);
    }

    pub fn remove_layer(&mut self, layer: SmartRef<Layer>) {
        for fixture in self.fixtures.values() {
            let _ = fixture.write(|fixture| {
                fixture.remove_layer_from_parameters(&layer);
            });
        }

        self.layers.retain(|l| l.uuid() != layer.uuid());
    }

    /// Evaluate current values of parameter layers for every fixture
    pub fn eval_parameters(&self) {
        for fixture in self.fixtures.values() {
            let _ = fixture.write(|fixture| {
                fixture.update_parameters();
            });
        }
    }
}
