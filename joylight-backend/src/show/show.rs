use anyhow::Context;
use crate::fixture::FixtureRef;
use std::{collections::HashMap, error::Error};
use crate::fixture::selection::Selection;
use crate::parameter::parameter_value::ParameterValue;
use std::sync::{Arc,Weak,RwLock};
use log::*;
use super::layer::*;

#[derive(Debug)]
#[derive(Default)]
pub struct Show {
    /// The fixtures that are currently in the show
    pub fixtures: HashMap<String, FixtureRef>,
    /// Weak pointers to active selections that should be updated when necessary
    selections: Vec<Weak<RwLock<dyn Selection>>>,
    layers: Vec<Layer>,
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

    pub fn add_layer(&mut self, layer: Layer) {
        self.layers.push(layer);
    }

    /// Evaluate current values of parameter layers
    /// 
    /// TODO: This creates and fills a new map for every iteration. This state can be stored and somewhat optimised.
    pub fn eval_layers(&self) {
        let mut parameters: HashMap<FixtureParameterPair, Vec<(&Layer,&ParameterValue)>> = HashMap::new();

        // Load active parameter values into map
        for layer in self.layers.iter() {
            if !layer.active {
                continue;
            }

            for (pair, value) in layer.parameters.iter() {
                parameters.entry(pair.clone()).or_default().push((layer, value));
            }
        }

        // Apply parameters to fixture
        // TODO: Blending modes/priorities
        // TODO: Make sure that the parameter value is the same type or convertible..
        for (pair, values) in parameters.iter() {
            pair.fixture.write(|fixture| {
                for (layer, value) in values {
                    let _ = fixture.set_parameter(pair.parameter, (*value).clone())
                        .with_context(|| format!("Layer {} setting parameter {} of {}", layer.name, pair.parameter, fixture.name))
                        .map_err(|e| error!("Error setting parameter: {}", e));
                }
            });
        }
    }
}