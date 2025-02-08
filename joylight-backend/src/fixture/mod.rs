//! Lighting fixtures

pub mod fixture_template;

use crate::fixture::fixture_template::FixtureTemplate;
use crate::parameter::parameter_value::ParameterValue;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

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

impl Fixture {
    /// Create a new fixture based on a template, setting parameter values to their defaults
    pub fn new(name: &str, template: FixtureTemplate) -> Fixture {
        let mut parameters = Vec::with_capacity(template.parameters.len());

        for parameter_type in template.parameters.iter() {
            let value = parameter_type.new_value();
            parameters.push(value);
        }

        return Fixture {
            name: name.to_string(),
            template,
            parameters,
        };
    }
}
