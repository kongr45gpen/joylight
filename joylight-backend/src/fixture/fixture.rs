use anyhow::{anyhow, Context, Result};
use crate::fixture::fixture_template::FixtureTemplate;
use crate::parameter::parameter_value::ParameterValue;
use crate::utils::{SmartRef, WithUuid};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::fmt::Debug;
use std::sync::{Arc, RwLock};

/// An instance of a fixture with multiple parameter values.
///
/// A show may have multiple fixture instances of the same [FixtureTemplate].
#[derive(Debug)]
pub struct Fixture {
    pub name: String,
    pub uuid: Uuid,
    pub template: FixtureTemplate,
    /// A vector of parameters, each associated to the [ParameterType] of the [FixtureTemplate]
    parameters: Vec<ParameterValue>,
}

/// Thread-safe reference to a fixture, to be passed around
pub type FixtureRef = SmartRef<Fixture>;

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
            uuid: Uuid::new_v4(),
            template: template.clone(),
            parameters,
        }
    }

    /// Gets the parameter index, if it exists, by its alias
    pub fn get_parameter_by_name(&self, name: &str) -> Option<usize> {
        self.template
            .parameters
            .iter()
            .zip(self.parameters.iter())
            .find(|(parameter_type, _)| parameter_type.alias == name)
            .iter().enumerate().next().map(|(index, _)| index)
    }

    pub fn get_parameter_by_number(&self, index: usize) -> Option<&ParameterValue> {
        self.parameters.get(index)
    }

    /// Get a list, in order, of all the parameter values
    pub fn get_parameter_values(&self) -> &Vec<ParameterValue> {
        self.parameters.as_ref()
    }

    pub fn set_parameter(&mut self, index: usize, value: ParameterValue) -> Result<()> {
        let parameter = self.parameters.get_mut(index)
            .ok_or_else(|| anyhow!("Parameter index out of bounds"))?;

        let parameter_type = self.template.parameters.get(index)
            .ok_or_else(|| anyhow!("Parameter type index out of bounds"))?;

        parameter_type.description.set_value(parameter, value)
            .with_context(|| format!("Setting parameter {} of {}", index, self.name))
    }
}

impl WithUuid for Fixture {
    fn uuid(&self) -> Uuid {
        self.uuid
    }
}