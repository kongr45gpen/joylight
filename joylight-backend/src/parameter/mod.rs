//! Each [fixture](crate::fixture) contains many parameters that describe the current, real-time state of the fixture.
//! This is similar, for example, to the definition of DMX channels.
//! 
//! As the representation of parameters differs significantly between what a human wants to see/modify, what is stored
//! internally, and what the fixture receives, we use an MVC-like pattern to allow flexibility in representing parameters.
//! 
//! ```text
//! /-------------------\     /-------------------\     /-------------------\
//! |   Parameter View  |     |                   |     | Parameter Encoder |
//! |     describes     |     |     Parameter     |     |     describes     |
//! |-------------------|     |       Value       |     |-------------------|
//! |     View Value    |     |                   |     |   Encoded Value   |
//! \-------------------/     \-------------------/     \-------------------/
//!           \-------------------------|-------------------------/
//!                          Linked by Parameter Type
//! ```
//! 
//! - **Views**: A parameter view is shown to the user and can be converted to/from a parameter value. The [parameter_view::ParameterView]
//! trait describes the view and can be anything from a group of numbers to an RGBA color picker. **The frontend sees only the view**
//! which is encoded in [parameter_view::ViewValue]. To support extensibility, views are typed as traits and should be used as such
//! (e.g. `Box<dyn ParameterView>`).
//! - **Models**: The internal model of the parameter, as stored in the backend, should correspond 1-1 to the mathematical representation
//! of the fixture's state. This is described by the [parameter_value::ParameterValue] enum.
//! - **Encoders**: The [parameter_dmx::ParameterEncoder] trait describes how the parameter value is mapped to the fixture's protocol
//! (such as DMX). The final encoded value then depends on the protocol used.
//! 
//! ## Example
//! Let's take the color of a CMY (Cyan, Magenta, Yellow) fixture as an example.
//! 
//! The user manual will describe the selected DMX channel assignment. For example, fixtures with fine control will assign 2 DMX channels
//! to each color. Therefore, our [ParameterEncoder](parameter_dmx::ParameterEncoder) will be a
//! [DMXMappingTransformer](parameter_dmx::DMXMappingTransformer) with `size = 2`.
//! 
//! As mentioned above, the most convenient way to store the value internally is to have it correspond to the actual fixture state.
//! In this case, we can store 3 floating-point numbers, ranging from `0` to `1`, in a [ParameterValue](parameter_value::ParameterValue).
//! [ParameterValue](parameter_value::ParameterValue) is an enum with different options, such as [Number](parameter_value::ParameterValue::Number)
//! or [Integer](parameter_value::ParameterValue::Integer), but the most suitable one for our situation is
//! [ColorBasedOnComponents](parameter_value::ColorBasedOnComponents). In our example we have 3 components (C, M, Y), and we must not forget to set
//! `subtractive = true` to select the proper color mixing method.
//! 
//! The users of the tool might be more familiar with the RGB model, or may want to use predefined hex-codes that need to be converted
//! to CMY. This is where a carefully selected [ParameterView](parameter_view::ParameterView) will come in handy.
//! [parameter_view::rgb] will return a [ColorComponentView](parameter_view::ColorComponentView) that describes the RGB representation,
//! and provides the necessary functions to convert to and from CMY.

pub mod parameter_dmx;
pub mod parameter_type;
pub mod parameter_value;
pub mod parameter_view;
