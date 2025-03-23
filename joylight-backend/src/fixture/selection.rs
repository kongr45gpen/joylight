use std::collections::{BTreeSet, HashSet};
use std::sync::{Arc, RwLock, Weak};

use crate::fixture::FixtureRef;
use crate::show::Show;

use super::Fixture;

pub trait Selection {
    fn name(&self) -> &str;
    fn fixtures(&self) -> &Vec<FixtureRef>;
    fn update(&mut self, show: &Show);
}

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

pub enum Filter {
    All,
    Not(Box<Filter>),
    And(Vec<Filter>),
    Or(Vec<Filter>),
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
        self.resolved_fixtures = show.fixtures.values()
            .filter(|fixture| self.filter.eval(fixture))
            .cloned()
            .collect();
    }
}

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

#[derive(Debug)]
pub struct DummySelection {}

impl Selection for DummySelection {
    fn name(&self) -> &str {
        "Dummy Selection"
    }

    fn fixtures(&self) -> &Vec<FixtureRef> {
        unimplemented!()
    }

    fn update(&mut self, _: &Show) {}
}