//! Parameter value containers

use crate::colors::RGBTuple;
use anyhow::{anyhow, Result};
use serde::{de, Deserialize, Serialize};
use std::fmt::Debug;

/// The current parameter value of a fixture.
///
/// It should correspond 1-1 to the fixture's parameter.
#[derive(Clone, Debug)]
pub enum ParameterValue {
    /// A floating-point parameter value
    Number(Vec<f64>),
    /// An integer parameter value (commonly used for enums)
    Integer(Vec<i64>),
}

/// A color based on addition or subtraction of hard-coded color components.
///
/// This is used for fixtures that represent their colour as a combination of light sources or filters.
/// This includes RGB, CMY and other combinations.
/// A representation on this struct should correspond 1-1 with the fixture's representation.
#[derive(Clone, Debug)]
pub struct ColorBasedOnComponents {
    /// The list of RGB components, in order, that are encoded for the fixture.
    /// Usually this will be RGB itself (i.e. `[[255,0,0] [0,255,0] [0,0,255]]`),
    pub components: Vec<RGBTuple>,
    /// [false] if the components are additive (e.g. RGB), [true] if they are subtractive (e.g. CMY)
    pub subtractive: bool,
}

#[derive(Clone, Debug)]
pub enum ParameterDescription {
    Number(usize),
    Integer(usize),
    ColorBasedOnComponents(ColorBasedOnComponents),
}

impl ParameterDescription {
    /// Check whether a `value` is compatible with this parameter description.
    pub fn check(&self, value: &ParameterValue) -> Result<()> {
        fn compare_lengths(expected: usize, actual: usize) -> Result<()> {
            if expected != actual {
                Err(anyhow!("Expected {} elements, got {}", expected, actual))
            } else {
                Ok(())
            }
        }

        fn incompatible_types() -> Result<()> {
            Err(anyhow!("Incompatible parameter value types"))
        }

        match self {
            ParameterDescription::Number(len) => {
                if let ParameterValue::Number(v) = value {
                    compare_lengths(*len, v.len())
                } else {
                    incompatible_types()
                }
            }
            ParameterDescription::Integer(len) => {
                if let ParameterValue::Integer(v) = value {
                    compare_lengths(*len, v.len())
                } else {
                    incompatible_types()
                }
            }
            ParameterDescription::ColorBasedOnComponents(components) => {
                if let ParameterValue::Number(v) = value {
                    compare_lengths(components.components.len(), v.len())
                } else {
                    incompatible_types()
                }
            }
        }
    }

    pub fn set_value(&self, target: &mut ParameterValue, new: ParameterValue) -> Result<()> {
        self.check(&new)?;

        *target = new;

        Ok(())
    }
}