//! Parameter value containers

use std::cmp::PartialOrd;
use std::fmt::Debug;
use std::ops::{Add, Mul};

use anyhow::{anyhow, Result};
use serde::{de, Deserialize, Serialize};

use crate::colors::RGBTuple;

/// The current parameter value of a fixture.
///
/// It should correspond 1-1 to the fixture's parameter.
#[derive(Clone, Debug, PartialEq)]
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

    /// Whether addition and multiplication are allowed on this parameter
    pub fn can_operate(&self) -> bool {
        match self {
            ParameterDescription::Number(_) => true,
            ParameterDescription::Integer(_) => false,
            ParameterDescription::ColorBasedOnComponents(_) => true,
        }
    }
}

impl Add for ParameterValue {
    type Output = Result<ParameterValue>;

    fn add(self, other: Self) -> Self::Output {
        match (self, other) {
            (ParameterValue::Number(a), ParameterValue::Number(b)) => {
                if a.len() != b.len() {
                    return Err(anyhow!("Incompatible lengths"));
                }

                Ok(ParameterValue::Number(
                    a.into_iter().zip(b.into_iter()).map(|(a, b)| a + b).collect(),
                ))
            }
            (ParameterValue::Integer(a), ParameterValue::Integer(b)) => {
                todo!("Integer operations")
            }
            _ => Err(anyhow!("Incompatible types")),
        }
    }
}

impl Mul for ParameterValue {
    type Output = Result<ParameterValue>;

    fn mul(self, other: Self) -> Self::Output {
        match (self, other) {
            (ParameterValue::Number(a), ParameterValue::Number(b)) => {
                if a.len() != b.len() {
                    return Err(anyhow!("Incompatible lengths"));
                }

                // Element-wise multiplication
                Ok(ParameterValue::Number(
                    a.into_iter().zip(b.into_iter()).map(|(a, b)| a * b).collect(),
                ))
            }
            (ParameterValue::Integer(a), ParameterValue::Integer(b)) => {
                todo!("Integer operations")
            }
            _ => Err(anyhow!("Incompatible types")),
        }
    }
}

impl PartialOrd for ParameterValue {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (ParameterValue::Number(a), ParameterValue::Number(b)) => {
                if a.len() != b.len() {
                    return None;
                }

                if a.len() == 1 {
                    return a[0].partial_cmp(&b[0]);
                }

                // Compare magnitudes based on Euclidean distance
                let abs_a = a.iter().fold(0.0, |acc, x| acc + x.powf(2.0));
                let abs_b = b.iter().fold(0.0, |acc, x| acc + x.powf(2.0));

                abs_a.partial_cmp(&abs_b)
            }
            _ => None,
        }
    }
}
