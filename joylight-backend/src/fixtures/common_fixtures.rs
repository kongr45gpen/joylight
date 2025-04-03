//! Simple fixtures and parameters for common use in examples, tests etc.

use std::sync::LazyLock;
use crate::colors::{blue, green, red};

use crate::parameters::ParameterType;

use super::FixtureTemplate;

pub const BRIGHTNESS: LazyLock<ParameterType> = LazyLock::new(|| ParameterType::new(
    "brightness",
    "Brightness",
    Box::new(crate::parameters::parameter_view::percentage()),
    Box::new(crate::parameters::parameter_encoding::DMXMappingTransformer {
        input_min: 0.0,
        input_max: 100.0,
        size: 1,
        endianness: crate::parameters::parameter_encoding::Endianness::Big,
    }),
    crate::parameters::parameter_value::ParameterValueDescription::Number(1),
    crate::parameters::parameter_value::ParameterValue::Number(vec![0.0]),
    None,
));

pub const COLOR_RGB: LazyLock<ParameterType> = LazyLock::new(|| ParameterType::new(
    "color_rgb",
    "RGB Color",
    Box::new(crate::parameters::parameter_view::rgb()),
    Box::new(crate::parameters::parameter_encoding::DMXMappingTransformer {
        input_min: 0.0,
        input_max: 255.0,
        size: 1,
        endianness: crate::parameters::parameter_encoding::Endianness::Big,
    }),
    crate::parameters::parameter_value::ParameterValueDescription::ColorBasedOnComponents(
        crate::parameters::parameter_value::ColorBasedOnComponents {
            components: vec![red(), green(), blue()],
            subtractive: false,
        },
    ),
    crate::parameters::parameter_value::ParameterValue::Number(vec![255.0, 255.0, 255.0]),
    None,
));

pub const DIMMER: LazyLock<FixtureTemplate> = LazyLock::new(|| FixtureTemplate {
    name: "Dimmer".to_string(),
    parameters: vec![BRIGHTNESS.clone()],
});

pub const TRIPLE_DIMMER: LazyLock<FixtureTemplate> = LazyLock::new(|| FixtureTemplate {
    name: "Triple Dimmer".to_string(),
    parameters: vec![BRIGHTNESS.clone(), BRIGHTNESS.clone(), BRIGHTNESS.clone()],
});

pub const RGB: LazyLock<FixtureTemplate> = LazyLock::new(|| FixtureTemplate {
    name: "RGB".to_string(),
    parameters: vec![COLOR_RGB.clone()],
});

pub const RGB_DIMMER: LazyLock<FixtureTemplate> = LazyLock::new(|| FixtureTemplate {
    name: "RGB Dimmer".to_string(),
    parameters: vec![BRIGHTNESS.clone(), COLOR_RGB.clone()],
});
