//! Lighting fixtures

pub mod fixture_template;
pub mod selection;

use crate::fixture::fixture_template::FixtureTemplate;
use crate::parameter::parameter_value::ParameterValue;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::sync::{Arc, RwLock};

/// An instance of a fixture with multiple parameter values.
///
/// A show may have multiple fixture instances of the same [FixtureTemplate].
#[derive(Debug)]
pub struct Fixture {
    pub name: String,
    pub template: FixtureTemplate,
    /// A vector of parameters, each associated to the [ParameterType] of the [FixtureTemplate]
    pub parameters: Vec<ParameterValue>,
}

/// Thread-safe reference to a fixture, to be passed around
pub type FixtureRef = Arc<RwLock<Fixture>>;

impl Fixture {
    /// Create a new fixture based on a template, setting parameter values to their defaults
    pub fn new(name: &str, template: &FixtureTemplate) -> Fixture {
        let parameters = template
            .parameters
            .iter()
            .map(|parameter_type| parameter_type.new_value())
            .collect();

        Fixture {
            name: name.to_string(),
            template: template.clone(),
            parameters,
        }
    }

    pub fn get_parameter_by_name(&mut self, name: &str) -> Option<&mut ParameterValue> {
        self.template
            .parameters
            .iter()
            .zip(self.parameters.iter_mut())
            .find(|(parameter_type, _)| parameter_type.alias == name)
            .map(|(_, value)| value)
    }
}
