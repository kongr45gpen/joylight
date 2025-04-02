//! Selections allow picking a static or dynamic subset of fixtures based on the user's whim.

use std::collections::{BTreeSet, HashSet};
use std::sync::{Arc, RwLock, Weak};

use super::Fixture;
use crate::fixtures::FixtureRef;
use crate::show::Show;
use crate::utils::WithUuid;

/// A selection describes a user-defined set of fixtures. It may be a fixed group of fixtures,
/// or updated dynamically based on some filter. For example, you can match fixtures of a
/// certain brand or with a certain name.
///
/// Selections are used to define fixture groups, to define event outputs, and wherever else
/// a dynamic set of fixtures is needed.
pub trait Selection {
    fn name(&self) -> &str;

    /// Get the list of fixtures represented by this selection. This may be outdated if
    /// [Selection::update] is not called after a fixture change.
    fn fixtures(&self) -> &Vec<FixtureRef>;

    /// Update the list of fixtures stored internally. Called when fixtures change.
    fn update(&mut self, show: &Show);
}

/// A selection where the list of fixtures is predefined
#[derive(Debug)]
pub struct StrictSelection {
    pub name: String,
    pub fixtures: Vec<FixtureRef>,
}

impl Selection for StrictSelection {
    fn name(&self) -> &str {
        &self.name
    }

    fn fixtures(&self) -> &Vec<FixtureRef> {
        &self.fixtures
    }

    fn update(&mut self, _: &Show) {}
}

/// A recursive filter for fixtures
pub enum Filter {
    All,
    Not(Box<Filter>),
    And(Vec<Filter>),
    Or(Vec<Filter>),
    /// A function that returns `true` if a fixture should be included in a given selection
    Predicate(Box<dyn Fn(&FixtureRef) -> bool>),
}

impl Filter {
    fn eval(&self, fixture: &FixtureRef) -> bool {
        match self {
            Filter::All => true,
            Filter::Not(filter) => !filter.eval(fixture),
            Filter::And(filters) => filters.iter().all(|filter| filter.eval(fixture)),
            Filter::Or(filters) => filters.iter().any(|filter| filter.eval(fixture)),
            Filter::Predicate(predicate) => predicate(fixture),
        }
    }
}

/// A selection where the list of fixtures is defined by a filter (or a combination of filters) and
/// evaluated dynamically
pub struct FilteredSelection {
    pub name: String,
    pub filter: Filter,
    pub resolved_fixtures: Vec<FixtureRef>,
}

impl FilteredSelection {
    pub fn new(name: &str, filter: Filter) -> FilteredSelection {
        FilteredSelection {
            name: name.to_string(),
            filter,
            resolved_fixtures: vec![],
        }
    }
}

impl Selection for FilteredSelection {
    fn name(&self) -> &str {
        &self.name
    }

    fn fixtures(&self) -> &Vec<FixtureRef> {
        &self.resolved_fixtures
    }

    fn update(&mut self, show: &Show) {
        self.resolved_fixtures = show
            .get_fixtures()
            .values()
            .filter(|fixture| self.filter.eval(fixture))
            .cloned()
            .collect();
    }
}

/// A selection which is a combination of one or more other selections
pub struct UnionSelection {
    pub name: String,
    pub selections: Vec<Weak<RwLock<dyn Selection>>>,
    pub resolved_fixtures: Vec<FixtureRef>,
}

impl UnionSelection {
    pub fn new(name: &str) -> UnionSelection {
        UnionSelection {
            name: name.to_string(),
            selections: vec![],
            resolved_fixtures: vec![],
        }
    }

    pub fn add_selection(&mut self, selection: Arc<RwLock<dyn Selection>>) {
        self.selections.push(Arc::downgrade(&selection));
    }
}

impl Selection for UnionSelection {
    fn name(&self) -> &str {
        &self.name
    }

    fn fixtures(&self) -> &Vec<FixtureRef> {
        unimplemented!()
    }

    fn update(&mut self, show: &Show) {
        unimplemented!()
    }
}

/// Used for debugging, this selection will throw an error when used
#[derive(Debug)]
pub(crate) struct DummySelection {}

impl Selection for DummySelection {
    fn name(&self) -> &str {
        "Dummy Selection"
    }

    fn fixtures(&self) -> &Vec<FixtureRef> {
        unimplemented!()
    }

    fn update(&mut self, _: &Show) {}
}

/// A selection built for easy addition/removal of fixtures
#[derive(Debug)]
pub struct MutableStrictSelection {
    pub name: String,
    pub fixtures: BTreeSet<FixtureRef>,
}

impl Selection for MutableStrictSelection {
    fn name(&self) -> &str {
        &self.name
    }

    fn fixtures(&self) -> &Vec<FixtureRef> {
        unimplemented!()
    }

    fn update(&mut self, _: &Show) {}
}
