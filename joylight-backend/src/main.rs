// OK to ignore during early development, to be removed later
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

mod colors;
mod effects;
mod fixture;
mod parameter;
mod show;

use std::any::TypeId;
use std::boxed::Box;
use std::collections::BTreeMap;

use joylight_backend::setup_logger;
use effects::{io::NodeDataset, node::NodeParameterValue};
use fixture::fixture_template::FixtureTemplate;
use fixture::Fixture;
use log::{debug, error, info, trace, warn};
use parameter::parameter_type::ParameterType;
use parameter::parameter_value::ParameterValue;
use parameter::parameter_view;

use serde_json::json;

use std::time::SystemTime;
use std::{thread, time};


fn main() {
    setup_logger();

    info!("Hello, world!");

   
}
