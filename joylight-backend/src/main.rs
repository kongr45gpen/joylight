mod fixture;
mod parameter;
mod fixture_template;

use std::collections::BTreeMap;
use std::boxed::Box;

use serde_json::json;
use std::{thread, time};

use zmq;

fn main() {
    println!("Hello, world!");

    let mut fixture1 = fixture::Fixture {
        name: "fixture1".to_string(),
        parameters: std::collections::BTreeMap::new(),
    };

    let brightness = parameter::Brightness { value: 1.0 };
    fixture1.parameters.insert("brightness".to_string(), Box::new(parameter::DMXParameterType::Brightness(brightness)));

    let mut fixture2 = fixture::Fixture {
        name: "fixture2".to_string(),
        parameters: std::collections::BTreeMap::new(),
    };
    let dimmer = parameter::Brightness { value: 0.5 };
    fixture2.parameters.insert("dimmer".to_string(), Box::new(parameter::DMXParameterType::Brightness(dimmer)));
    let color = parameter::Color { r: 1.0, g: 0.0, b: 0.0 };
    fixture2.parameters.insert("color".to_string(), Box::new(parameter::DMXParameterType::Color(color)));

    let mut templates = Vec::new();

    templates.push(fixture_template::FixtureTemplate {
        name: "Dimmer".to_string(),
        parameters: BTreeMap::from([
            ("brightness".to_string(), "uint8".to_string())
        ])
    });

    templates.push(fixture_template::FixtureTemplate {
        name: "RGB".to_string(),
        parameters: BTreeMap::from([
            ("red".to_string(), "uint8".to_string()),
            ("green".to_string(), "uint8".to_string()),
            ("blue".to_string(), "uint8".to_string())
        ])
    });

    templates.push(fixture_template::FixtureTemplate {
        name: "RGBW".to_string(),
        parameters: BTreeMap::from([
            ("red".to_string(), "uint8".to_string()),
            ("green".to_string(), "uint8".to_string()),
            ("blue".to_string(), "uint8".to_string()),
            ("white".to_string(), "uint8".to_string()),
        ])
    });

    templates.push(fixture_template::FixtureTemplate {
        name: "DRGB".to_string(),
        parameters: BTreeMap::from([
            ("brightness".to_string(), "uint8".to_string()),
            ("red".to_string(), "uint8".to_string()),
            ("green".to_string(), "uint8".to_string()),
            ("blue".to_string(), "uint8".to_string()),
        ])
    });

    templates.push(fixture_template::FixtureTemplate {
        name: "Genericbrand Mover".to_string(),
        parameters: BTreeMap::from([
            ("brightness".to_string(), "uint8".to_string()),
            ("red".to_string(), "uint8".to_string()),
            ("green".to_string(), "uint8".to_string()),
            ("blue".to_string(), "uint8".to_string()),
            ("pan".to_string(), "uint16".to_string()),
            ("tilt".to_string(), "uint16".to_string()),
            ("gobo_static".to_string(), "1-10: circle 11-20: star 21-30: rectange 31-255: off".to_string()),
            ("gobo_rotating".to_string(), "1-10: clouds 11-20: lines 21-30: tree 31-255: off".to_string()),
            ("gobo_speed".to_string(), "uint8".to_string()),
            ("strobe".to_string(), "0-100: off 101-255: speed".to_string()),
            ("focus".to_string(), "uint8".to_string()),
            ("zoom".to_string(), "uint8".to_string()),
            ("mode".to_string(), "0-10: ok 11-14: reset 15-19: brtdown 20-24: brtup 25-29: park".to_string()),
        ])
    });

    templates.push(fixture_template::FixtureTemplate {
        name: "Genericbrand LED Wash Mover".to_string(),
        parameters: BTreeMap::from([
            ("red".to_string(), "uint8".to_string()),
            ("green".to_string(), "uint8".to_string()),
            ("blue".to_string(), "uint8".to_string()),
            ("coldwhite".to_string(), "uint8".to_string()),
            ("warmwhite".to_string(), "uint8".to_string()),
            ("pan".to_string(), "uint16".to_string()),
            ("tilt".to_string(), "uint16".to_string()),
            ("focus".to_string(), "uint8".to_string()),
        ])
    });

    templates.push(fixture_template::FixtureTemplate {
        name: "Genericbrand Triple LED".to_string(),
        parameters: BTreeMap::from([
            ("red_1".to_string(), "uint8".to_string()),
            ("green_1".to_string(), "uint8".to_string()),
            ("blue_1".to_string(), "uint8".to_string()),
            ("red_2".to_string(), "uint8".to_string()),
            ("green_2".to_string(), "uint8".to_string()),
            ("blue_2".to_string(), "uint8".to_string()),
            ("red_3".to_string(), "uint8".to_string()),
            ("green_3".to_string(), "uint8".to_string()),
            ("blue_3".to_string(), "uint8".to_string()),
        ])
    });

    println!("{:#?}", templates);


    let context = zmq::Context::new();
    let server = context.socket(zmq::REP).unwrap();
    server.bind("tcp://*:5555").unwrap();

    println!("ZeroMQ connection started");

    let json_str = json!({
        // Fixture templates, i.e. fixtures that _can_ be added to the show
        "templates": templates,
        // Fixtures, i.e. fixtures that are already in the show. (TODO each fixture should correspond to one template)
        "fixtures": vec![fixture1, fixture2]
    }).to_string();

    loop {
        let string = server.recv_string(0).unwrap().unwrap();
        println!("Received request: {}", string);

        thread::sleep(time::Duration::from_millis(100));

        server.send(json_str.as_str(), 0).unwrap();
    }
}
