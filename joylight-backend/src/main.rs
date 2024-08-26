#![allow(unused_imports)]
#![allow(dead_code)]

mod fixture;
mod parameter;
mod fixture_template;

use std::collections::BTreeMap;
use std::boxed::Box;

use fixture_template::FixtureTemplate;
use parameter::ColorRecord;
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

    {
        let rgb = parameter::RGB {
            order: parameter::RGBOrder::BGR,
        };

        let record: <parameter::RGB as ParameterType>::Record = parameter::ColorRecord {
            color: [0.1, 0.2, 0.3],
        };

        let value = rgb.get_value(&record);
        println!("{:#?}: {:?}", record, value);
    }

    let mut ft = FixtureTemplate {
        name: "Test".to_string(),
        parameters: BTreeMap::new(),
    };

    ft.parameters.insert("brightness".to_string(), Box::new(parameter::Brightness {
        size: 1,
        endianness: parameter::ByteOrder::BigEndian,
    }));
    ft.parameters.insert("color".to_string(), Box::new(parameter::RGB {
        order: parameter::RGBOrder::BGR,
    }));

    let myval:f32 = 0.5;
    let mycolor = ColorRecord {
        color: [0.1, 0.2, 0.3],
    };

    let brt_dmx = ft.parameters["brightness"].get_value(&myval);
    println!("BRT value: {:?}", brt_dmx);

    let color_dmx = ft.parameters["color"].get_value(&mycolor);
    println!("Color value: {:?}", color_dmx);
}
