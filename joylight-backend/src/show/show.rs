use std::collections::{BTreeMap, HashMap};
use std::error::Error;
use std::sync::{Arc, RwLock, Weak};

use anyhow::Context;
use log::*;
use uuid::Uuid;

use super::{layer::*, Scene};
use crate::fixtures::FixtureRef;
use crate::fixtures::selection::Selection;
use crate::parameters::parameter_value::ParameterValue;
use crate::utils::{SmartRef, WeakRef, WithUuid};

#[derive(Debug, Default)]
pub struct Show {
    /// The fixtures that are currently in the show
    fixtures: BTreeMap<Uuid, FixtureRef>,
    /// Weak pointers to active selections that should be updated when necessary
    selections: Vec<WeakRef<dyn Selection>>,
    /// A list of all layers running in the show
    layers: Vec<SmartRef<Layer>>,
    /// The program is a high-priority scene where the user edits
    pub program: Scene,
}

impl Show {
    /// Add a fixture to the show. This will be used for display, selection updates and more.
    pub fn add_fixture(&mut self, fixture: FixtureRef) {
        self.fixtures.insert(fixture.uuid(), fixture);
    }

    /// Remove a fixture by UUID
    pub fn remove_fixture(&mut self, fixture: FixtureRef) {
        self.fixtures.remove(&fixture.uuid());
    }

    /// Get a list of all fixtures
    pub fn get_fixtures(&self) -> &BTreeMap<Uuid, FixtureRef> {
        &self.fixtures
    }

    /// Add a selection to the list of selections to be updated whenever fixture parameters change.
    pub fn add_selection(&mut self, selection: &SmartRef<dyn Selection>) {
        self.selections.push(WeakRef::from_smartref(selection));
    }

    /// Callback to refresh fixtures in selections
    pub fn refresh_fixtures(&mut self) {
        for selection in self.selections.iter().filter_map(|weak| weak.get()) {
            let guard = selection.write(|sel| sel.update(self)).ok();
        }

        // Remove dangling selections (garbage collect...)
        // This may not be the fastest way but it is fully safe
        self.selections.retain(|weak| weak.strong_count() > 0);
    }

    /// Adds a layer to the show
    ///
    /// This function itself does not have any effect on the show, but it's important for
    /// record keeping.
    pub fn add_layer(&mut self, layer: SmartRef<Layer>) {
        self.layers.push(layer);
    }

    /// Removes a layer and its effect from all fixtures and parameters
    pub fn remove_layer(&mut self, layer: SmartRef<Layer>) {
        for fixture in self.fixtures.values() {
            let _ = fixture.write(|fixture| {
                fixture.remove_layer_from_parameters(&layer);
            });
        }

        self.layers.retain(|l| l.uuid() != layer.uuid());
    }

    /// Evaluate current values of parameter layers for every fixture.
    ///
    /// This will change the parameter values of every fixture based on the curretn layer values.
    pub fn eval_parameters(&self) {
        for fixture in self.fixtures.values() {
            let _ = fixture.write(|fixture| {
                fixture.update_parameters();
            });
        }
    }
}
