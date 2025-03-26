//! Lighting fixture templates

use crate::parameters::parameter_type::ParameterType;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt::Debug;

/// The template used to describe a fixture's name and parameters
#[derive(Clone, Debug)]
pub struct FixtureTemplate {
    pub name: String,
    pub parameters: Vec<ParameterType>,
}
