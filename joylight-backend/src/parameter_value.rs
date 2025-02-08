use std::{collections::BTreeMap, fmt::Debug};
use serde::{de, Deserialize, Serialize};
use crate::colors::RGBTuple;
use std::any::Any;
use dyn_clone::DynClone;

// /// Represents a color in the CIE XYZ color space
// #[derive(Clone, Debug)]
// pub struct Color {
//     pub x: f64,
//     pub y: f64,
//     pub z: f64,
//     /// Color quality (from 0 to 1)
//     pub q: f64
// }

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

#[derive(Clone, Debug)]
pub enum ParameterValue {
    Number(Vec<f64>),
    Integer(Vec<i64>),
    ColorBasedOnComponents(ColorBasedOnComponents),
}

