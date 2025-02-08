//! Parameter value containers

use std::fmt::Debug;
use serde::{de, Deserialize, Serialize};
use crate::colors::RGBTuple;

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
    /// The values of the components from 0 to 1 in order
    pub values: Vec<f64>,
    /// [false] if the components are additive (e.g. RGB), [true] if they are subtractive (e.g. CMY)
    pub subtractive: bool,
}

/// The current parameter value of a fixture.
/// 
/// This enum is used as a value container and does not always describe what this value is connected to.
/// 
/// It should correspond 1-1 to the fixture's parameter.
#[derive(Clone, Debug)]
pub enum ParameterValue {
    /// A floating-point parameter value
    Number(Vec<f64>),
    /// An integer parameter value (commonly used for enums)
    Integer(Vec<i64>),
    ColorBasedOnComponents(ColorBasedOnComponents),
}

