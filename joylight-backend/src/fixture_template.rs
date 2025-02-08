use std::collections::BTreeMap;
use std::fmt::Debug;
use serde::{Deserialize, Serialize};
use crate::parameter_type::ParameterType;

#[derive(Clone, Debug)]
pub struct FixtureTemplate {
    pub name: String,
    pub parameters: Vec<ParameterType>,
}