//! A view of a parameter's value, designed to be readable and editable by the user

use crate::{
    colors,
    colors::{ColorModel, RGBTuple},
    parameter::parameter_value::ParameterValue,
};
use dyn_clone::DynClone;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

/// Protocol input value that may correspond to a view.
#[derive(Clone, Debug)]
pub enum ViewValue {
    F64(f64),
    I64(i64),
    String(String),
}

/// A parameter view represents the user-editable representation of a parameter's type and value.
///
/// A parameter could be associated with different types of views, but only one can be active at any time.
///
/// It can be reliably converted to and from the corresponding parameter value
pub trait ParameterView: DynClone + Debug {
    fn to_value(&self, input: Vec<ViewValue>, value: &mut ParameterValue) -> Result<(), ()> {
        Err(())
    }

    fn from_value(&self, value: &ParameterValue) -> Result<Vec<ViewValue>, ()> {
        Err(())
    }
}

/// A single scalar slider
#[derive(Clone, Debug)]
pub struct SliderView {
    pub min: f64,
    pub max: f64,
    pub step: f64,
    pub unit: String,
}

impl ParameterView for SliderView {
    fn from_value(&self, value: &ParameterValue) -> Result<Vec<ViewValue>, ()> {
        let ParameterValue::Number(numbers) = value else {
            return Err(());
        };

        numbers
            .first()
            .map(|number| vec![ViewValue::F64(*number)])
            .ok_or(())
    }

    fn to_value(&self, input: Vec<ViewValue>, value: &mut ParameterValue) -> Result<(), ()> {
        let ParameterValue::Number(numbers) = value else {
            return Err(());
        };

        if let Some(ViewValue::F64(number)) = input.first() {
            numbers[0] = *number;
            Ok(())
        } else {
            Err(())
        }
    }
}

/// A "joystick"-type view that allows rotating a fixture around two axes with the same unit
#[derive(Clone, Debug)]
pub struct Rotation2DView {
    pub pan_min: f64,
    pub pan_max: f64,
    pub tilt_min: f64,
    pub tilt_max: f64,
    pub unit: String,
}

// impl ParameterView for Rotation2DView {
// }

/// A basic percentage slider from 0% to 100%
pub fn percentage() -> SliderView {
    SliderView {
        min: 0.0,
        max: 100.0,
        step: 0.0,
        unit: "%".to_string(),
    }
}

/// A basic angle slider from 0° to 360°
pub fn circle() -> SliderView {
    SliderView {
        min: 0.0,
        max: 360.0,
        step: 0.0,
        unit: "°".to_string(),
    }
}

/// A view of a color parameter where the end color is made up from an addition (or subtraction) of
/// simpler component colors.
///
/// RGB, RGBW, RGBQ, CMY and others are typical examples of views supported by this structure.
///
/// This view does not have to actually correspond to the one used internally by the fixture. A simple
/// or complex conversion will take place if needed. This allows the user, for example, to set the
/// colors of an RGBAW fixture with an RGB view. The view itself is fixture-agnostic, while the conversion
/// functions and the parameter value are tied to the fixture.
#[derive(Clone, Debug)]
pub struct ColorComponentView {
    /// A list of RGB components that are included in the view.
    /// Usually this will be RGB itself (i.e. `[[255,0,0] [0,255,0] [0,0,255]]`),
    pub components: Vec<RGBTuple>,
    /// [false] if the components are additive (e.g. RGB), [true] if they are subtractive (e.g. CMY)
    pub subtractive: bool,
    /// Whether the view includes a Quality slider for balance control when the output components
    /// are more than the input components
    pub has_quality: bool,
    /// A human-readable description of the color space
    pub name: String,
}

impl ParameterView for ColorComponentView {}

pub fn rgb() -> ColorComponentView {
    ColorComponentView {
        components: vec![
            colors::red(),
            colors::green(),
            colors::blue(),
        ],
        subtractive: false,
        has_quality: false,
        name: "RGB".to_string(),
    }
}

// /// The standard RGB view
// pub static RGB: ColorComponentView = ColorComponentView {
//     components: vec![
//         RGBTuple([255, 0, 0]),
//         RGBTuple([0, 255, 0]),
//         RGBTuple([0, 0, 255]),
//     ],
//     subtractive: false,
//     has_quality: false,
//     name: "RGB".to_string(),
// };

// /// The standard RGB + Quality view
// pub static RGBQ: ColorComponentView = ColorComponentView {
//     components: vec![
//         RGBTuple([255, 0, 0]),
//         RGBTuple([0, 255, 0]),
//         RGBTuple([0, 0, 255]),
//     ],
//     subtractive: false,
//     has_quality: true,
//     name: "RGBQ".to_string(),
// };

// /// The standard RGBW view
// pub static RGBW: ColorComponentView = ColorComponentView {
//     components: vec![
//         RGBTuple([255, 0, 0]),
//         RGBTuple([0, 255, 0]),
//         RGBTuple([0, 0, 255]),
//         RGBTuple([255, 255, 255]),
//     ],
//     subtractive: false,
//     has_quality: false,
//     name: "RGBW".to_string(),
// };

// /// The standard subtractive CMY view
// pub static CMY: ColorComponentView = ColorComponentView {
//     components: vec![
//         RGBTuple([255, 0, 0]),
//         RGBTuple([0, 255, 0]),
//         RGBTuple([0, 0, 255]),
//     ],
//     subtractive: true,
//     has_quality: false,
//     name: "CMY".to_string(),
// };

/// A more generic view supporting more generic [ColorModel]s that cannot be expressed
/// as sums of components
#[derive(Clone, Debug)]
pub struct ColorModelView {
    /// The color model that this view represents
    pub model: ColorModel,
    pub has_quality: bool,
    pub name: String,
}

impl ParameterView for ColorModelView {}

pub fn hsv() -> ColorModelView {
    ColorModelView {
        model: ColorModel::HSV,
        has_quality: false,
        name: "HSV".to_string(),
    }
}

pub fn hsl() -> ColorModelView {
    ColorModelView {
        model: ColorModel::HSL,
        has_quality: false,
        name: "HSL".to_string(),
    }
}
