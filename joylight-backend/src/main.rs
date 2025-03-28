// OK to ignore during early development, to be removed later
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

mod colors;
mod effects;
mod fixtures;
mod parameters;
mod show;
mod utils;

use std::any::TypeId;
use std::boxed::Box;
use std::collections::BTreeMap;
use std::time::SystemTime;
use std::{thread, time};

use effects::io::NodeDataset;
use effects::node::NodeParameterValue;
use fixtures::{Fixture, FixtureTemplate};
use joylight_backend::setup_logger;
use log::{debug, error, info, trace, warn};
use parameters::parameter_type::ParameterType;
use parameters::parameter_value::ParameterValue;
use parameters::parameter_view;
use serde_json::json;

fn main() {
    setup_logger();

    info!("Hello, world!");
}
