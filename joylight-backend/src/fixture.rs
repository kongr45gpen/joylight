use std::collections::BTreeMap;
use std::boxed::Box;
use std::fmt::Debug;
use crate::fixture_template::FixtureTemplate;
use crate::parameter_type;
use crate::parameter_value;
use crate::parameter_value::ParameterValue;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Fixture {
    pub name: String,
    pub template: FixtureTemplate,
    pub parameters: Vec<ParameterValue>
}

impl Fixture {
    pub fn new(name: &str, template: FixtureTemplate) -> Fixture {
        let mut parameters = Vec::with_capacity(template.parameters.len());

        for parameter_type in template.parameters.iter() {
            let value = parameter_type.new_value();
            parameters.push(value);
        }

        return Fixture {
            name: name.to_string(),
            template,
            parameters,
        }
    }
}