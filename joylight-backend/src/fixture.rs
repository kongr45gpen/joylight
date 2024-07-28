use std::collections::BTreeMap;
use std::boxed::Box;
use std::fmt::Debug;
use crate::parameter::{DMXParameter, DMXParameterType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct Fixture {
    pub name: String,
    pub parameters: BTreeMap<String, Box<DMXParameterType>>,
}