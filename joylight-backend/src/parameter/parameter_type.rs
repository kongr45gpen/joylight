//! Generic descriptions of parameters corresponding to a fixture

use crate::parameter::{
    parameter_dmx::ParameterEncoder, parameter_value::ParameterValue, parameter_view::ParameterView,
};
use serde::{Deserialize, Serialize};
use std::any::{Any, TypeId};
use std::fmt::Debug;

/// Representation of a Parameter Type of a fixture.
///
/// A parameter represents any property of a fixture that can be controlled in real time, such
/// as brightness, color, position, etc. Usually it can refer to a DMX channel or another variable
/// of some kind.
///
/// Does not actually contain the value of the parameter, see [crate::parameter_value::ParameterValue].
#[derive(Debug)]
pub struct ParameterType {
    /// Internal unique parameter name
    pub alias: String,
    /// Parameter name shown to the user
    pub display_name: String,
    /// The user-readable representation of the parameter's value
    pub view: Box<dyn ParameterView>,
    /// Encoding (e.g. DMX) details
    pub encoding: Box<dyn ParameterEncoder>,
    /// Default value of the parameter on fixture initialisation.
    /// This basically also defines the type of the value itself.
    pub default_value: ParameterValue,
    /// Optional text comments/description shown to the user
    pub comments: Option<String>,
}

impl ParameterType {
    /// Create a new parameter type
    pub fn new(
        alias: &str,
        display_name: &str,
        view: Box<dyn ParameterView>,
        encoding: Box<dyn ParameterEncoder>,
        default_value: ParameterValue,
        comments: Option<String>,
    ) -> Self {
        Self {
            alias: alias.to_string(),
            display_name: display_name.to_string(),
            view: view,
            encoding: encoding,
            default_value: default_value,
            comments: comments,
        }
    }

    /// Instantiate the default parameter value from a parameter type
    pub fn new_value(&self) -> ParameterValue {
        // return dyn_clone::clone_box(&*self.default_value);
        return self.default_value.clone();
    }
}

impl Clone for ParameterType {
    fn clone(&self) -> Self {
        Self {
            alias: self.alias.clone(),
            display_name: self.display_name.clone(),
            view: dyn_clone::clone_box(&*self.view),
            encoding: dyn_clone::clone_box(&*self.encoding),
            default_value: self.default_value.clone(),
            comments: self.comments.clone(),
        }
    }
}
