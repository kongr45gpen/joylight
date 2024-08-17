use std::collections::BTreeMap;
use std::fmt::Debug;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct FixtureTemplate {
    pub name: String,
    pub parameters: BTreeMap<String, String>,
}