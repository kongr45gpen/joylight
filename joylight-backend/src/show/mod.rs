//! The show contains all information about the current state of the application, including
//! fixtures, programs, effects and other options.

pub mod layer;
pub use layer::*;

use crate::fixture::FixtureRef;
use std::{collections::HashMap, error::Error};
use crate::fixture::selection::Selection;
use std::sync::{Arc,Weak,RwLock};
use log::*;

pub struct Show {
    /// The fixtures that are currently in the show
    pub fixtures: HashMap<String, FixtureRef>,
    /// Weak pointers to active selections that should be updated when necessary
    selections: Vec<Weak<RwLock<dyn Selection>>>,
}

impl Default for Show {
    fn default() -> Self {
        Show {
            fixtures: HashMap::new(),
            selections: Vec::new(),
        }
    }
}

impl Show {
    pub fn add_fixture(&mut self, fixture: FixtureRef) {
        let name = fixture.read().unwrap().name.clone();
        self.fixtures.insert(name, fixture);
    }
    
    pub fn add_selection(&mut self, selection: Arc<RwLock<dyn Selection>>) {
        self.selections.push(Arc::downgrade(&selection));
    }

    pub fn refresh_fixtures(&mut self) {
        for selection in self.selections.iter().map(|weak| weak.upgrade()).flatten() {
            let guard = selection.write();
            if let Ok(mut selection) = guard {
                selection.update(&self);
            } else if let Err(e) = guard {
                error!("Failed to lock selection for update: {}", e);
            }
        }

        // Remove dangling selections (garbage collect...)
        // This may not be the fastest way but it is fully safe
        self.selections.retain(|weak| weak.strong_count() > 0);
    }
}