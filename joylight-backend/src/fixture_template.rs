use std::collections::BTreeMap;
use std::fmt::Debug;
use serde::{Deserialize, Serialize};
use crate::parameter::{DMXParameter, ParameterType};
use std::any::Any;


#[derive(Debug)]
pub struct FixtureTemplate {
    pub name: String,
    pub parameters: BTreeMap<String, Box<dyn Any>>,
}