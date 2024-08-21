#![allow(unused_imports)]

mod fixture;
mod parameter;
mod fixture_template;

use std::collections::BTreeMap;
use std::boxed::Box;

use serde_json::json;
use std::{thread, time};
use crate::parameter::{DMXParameter, ParameterType, Brightness};

use zmq;

fn main() {
    println!("Hello, world!");

    {
        let brightness = parameter::Brightness {
            size: 1,
            endianness: parameter::ByteOrder::BigEndian,
        };

        let record: <Brightness as ParameterType>::Record = 0.5;

        let value = brightness.get_value(&record);
        println!("{:?}", value);
    }

    {
        let brightness = parameter::Brightness {
            size: 2,
            endianness: parameter::ByteOrder::LittleEndian,
        };

        let record: <Brightness as ParameterType>::Record = 0.4;

        let value = brightness.get_value(&record);
        println!("{:?}", value);
    }
}
