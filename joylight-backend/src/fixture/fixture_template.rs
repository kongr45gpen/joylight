//! Lighting fixture templates

use std::collections::BTreeMap;
use std::fmt::Debug;
use serde::{Deserialize, Serialize};
use crate::parameter::parameter_type::ParameterType;

/// The template used to describe a fixture's name and parameters
#[derive(Clone, Debug)]
pub struct FixtureTemplate {
    pub name: String,
    pub parameters: Vec<ParameterType>,
}