use std::fmt::Debug;
use std::iter::Map;
use std::sync::{Arc, RwLock};
use std::time::Instant;

use anyhow::{anyhow, Context, Result};
use log::warn;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::fixtures::fixture_template::FixtureTemplate;
use crate::parameters::parameter_value::ParameterValue;
use crate::parameters::{ParameterRuntime, ParameterValueDescription, ViewValue};
use crate::show::{make_decision, Layer};
use crate::utils::{SmartRef, WithUuid};

/// An instance of a fixture with multiple parameter values.
///
/// A show may have multiple fixture instances of the same [FixtureTemplate].
#[derive(Debug)]
pub struct Fixture {
    pub name: String,
    pub uuid: Uuid,
    pub template: FixtureTemplate,
    /// A vector of parameters, each associated to the [ParameterType] of the [FixtureTemplate]
    parameters: Vec<ParameterRuntime>,
}

/// Thread-safe reference to a fixture, to be passed around
pub type FixtureRef = SmartRef<Fixture>;

impl Fixture {
    /// Create a new fixture based on a template, setting parameter values to their defaults
    pub fn new(name: &str, template: &FixtureTemplate) -> Fixture {
        let parameters = template
            .parameters
            .iter()
            .map(|parameter_type| ParameterRuntime::new_from_type(parameter_type))
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
            .iter()
            .enumerate()
            .next()
            .map(|(index, _)| index)
    }

    pub fn get_parameter_by_number(&self, number: usize) -> Option<&ParameterRuntime> {
        self.parameters.get(number)
    }

    pub fn get_parameter_by_number_mut(&mut self, number: usize) -> Option<&mut ParameterRuntime> {
        self.parameters.get_mut(number)
    }

    // pub fn get_parameter_by_number(&self, index: usize) -> Option<&ParameterValue> {
    //     self.parameters.get(index)
    // }

    /// Get a list, in order, of all the parameter values
    ///
    /// Useful for debugging
    pub fn get_parameters(&self) -> &Vec<ParameterRuntime> {
        self.parameters.as_ref()
    }

    pub fn get_parameter_values(&self) -> impl Iterator<Item = &ParameterValue> {
        self.parameters
            .iter()
            .map(|parameter| &parameter.value)
    }

    /// Set a parameter value directly
    ///
    /// This overrides a parameter value immediately. However, it's recommended to use [Layer]s within a show to
    /// properly set parameters based on priorities.
    pub fn set_parameter(&mut self, index: usize, value: ParameterValue) -> Result<()> {
        let parameter = self
            .parameters
            .get_mut(index)
            .ok_or_else(|| anyhow!("Parameter index out of bounds"))?;

        let parameter_type = self
            .template
            .parameters
            .get(index)
            .ok_or_else(|| anyhow!("Parameter type index out of bounds"))?;

        parameter_type
            .description
            .set_value(&mut parameter.value, value)
            .with_context(|| format!("Setting parameter {} of {}", index, self.name))
    }

    /// Update all parameters of the fixture based on the the [ParameterUpdate]s in each [ParameterRuntime]
    pub fn update_parameters(&mut self) {
        for (idx, parameter) in self.parameters.iter_mut().enumerate() {
            if !parameter.up_to_date  {
                continue;
            }

            // TODO: There is a better way to pass this without having to convert to a Vec
            let values = parameter.updates.iter().map(|(_, update)| update).cloned().collect::<Vec<_>>();

            let decision = make_decision(&values);

            if let Some(value) = decision {
                parameter.value = value;
            } else {
                // Value could not be generated, e.g. due to there being no updates on active layers.
                // Go back to the default value.
                let result = self.template.parameters.get(idx)
                    .map(|ptype| parameter.value = ptype.default_value.clone());

                if result.is_none() {
                    warn!("No default value for parameter {} of fixture {}", idx, self.name);
                }
            }

            parameter.up_to_date = true;
        }
    }

    pub fn remove_layer_from_parameters(&mut self, layer: &SmartRef<Layer>) {
        for parameter in self.parameters.iter_mut() {
            parameter.updates.remove(&layer.uuid());
        }
    }
}

impl WithUuid for Fixture {
    fn uuid(&self) -> Uuid {
        self.uuid
    }
}
