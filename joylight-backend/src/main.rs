mod colors;
mod fixture;
mod parameter;
mod effects;

use std::any::TypeId;
use std::boxed::Box;
use std::collections::BTreeMap;

use effects::{io::NodeDataset, node::NodeParameterValue};
use fixture::fixture_template::FixtureTemplate;
use fixture::Fixture;
use parameter::parameter_dmx;
use parameter::parameter_type::ParameterType;
use parameter::parameter_value::ParameterValue;
use parameter::parameter_view;
use parameter::parameter_view::ViewValue;
use serde_json::json;
use smallvec::smallvec;
use std::{thread, time};

use zmq;

fn main() {
    println!("Hello, world!");

    let brightness = ParameterType::new(
        "brightness",
        "Brightness",
        Box::new(parameter_view::percentage()),
        Box::new(parameter_dmx::DMXMappingTransformer {
            input_min: 0.0,
            input_max: 100.0,
            size: 1,
            endianness: parameter_dmx::Endianness::Big,
        }),
        ParameterValue::Number(vec![0.0]),
        None,
    );

    let color_rgb = ParameterType::new(
        "color_rgb",
        "RGB Color",
        Box::new(parameter_view::rgb()),
        Box::new(parameter_dmx::DMXMappingTransformer {
            input_min: 0.0,
            input_max: 255.0,
            size: 1,
            endianness: parameter_dmx::Endianness::Big,
        }),
        ParameterValue::Number(vec![255.0, 255.0, 255.0]),
        None,
    );

    let three_fixture_template = FixtureTemplate {
        name: "ThreeFixture".to_string(),
        parameters: vec![
            brightness.clone(),
            brightness.clone(),
            color_rgb.clone(),
            color_rgb.clone(),
        ],
    };

    let mut three_fixture = Fixture::new("BabisOFlou", three_fixture_template.clone());

    println!("{:?}", three_fixture);

    three_fixture_template.parameters[0].view.to_value(
        vec![parameter_view::ViewValue::F64(50.0)],
        &mut three_fixture.parameters[0],
    );
    println!(
        "{:?}",
        three_fixture_template.parameters[0]
            .view
            .from_value(&three_fixture.parameters[0])
    );

    // let mut fixture1 = fixture::Fixture {
    //     name: "fixture1".to_string(),
    //     parameters: std::collections::BTreeMap::new(),
    // };

    // let brightness = parameter::Brightness { value: 1.0 };
    // fixture1.parameters.insert("brightness".to_string(), Box::new(parameter::DMXParameterType::Brightness(brightness)));

    // let mut fixture2 = fixture::Fixture {
    //     name: "fixture2".to_string(),
    //     parameters: std::collections::BTreeMap::new(),
    // };
    // let dimmer = parameter::Brightness { value: 0.5 };
    // fixture2.parameters.insert("dimmer".to_string(), Box::new(parameter::DMXParameterType::Brightness(dimmer)));
    // let color = parameter::Color { r: 1.0, g: 0.0, b: 0.0 };
    // fixture2.parameters.insert("color".to_string(), Box::new(parameter::DMXParameterType::Color(color)));

    // let mut templates = Vec::new();

    // templates.push(fixture_template::FixtureTemplate {
    //     name: "Dimmer".to_string(),
    //     parameters: BTreeMap::from([
    //         ("brightness".to_string(), "uint8".to_string())
    //     ])
    // });

    // templates.push(fixture_template::FixtureTemplate {
    //     name: "RGB".to_string(),
    //     parameters: BTreeMap::from([
    //         ("red".to_string(), "uint8".to_string()),
    //         ("green".to_string(), "uint8".to_string()),
    //         ("blue".to_string(), "uint8".to_string())
    //     ])
    // });

    // templates.push(fixture_template::FixtureTemplate {
    //     name: "RGBW".to_string(),
    //     parameters: BTreeMap::from([
    //         ("red".to_string(), "uint8".to_string()),
    //         ("green".to_string(), "uint8".to_string()),
    //         ("blue".to_string(), "uint8".to_string()),
    //         ("white".to_string(), "uint8".to_string()),
    //     ])
    // });

    // templates.push(fixture_template::FixtureTemplate {
    //     name: "DRGB".to_string(),
    //     parameters: BTreeMap::from([
    //         ("brightness".to_string(), "uint8".to_string()),
    //         ("red".to_string(), "uint8".to_string()),
    //         ("green".to_string(), "uint8".to_string()),
    //         ("blue".to_string(), "uint8".to_string()),
    //     ])
    // });

    // templates.push(fixture_template::FixtureTemplate {
    //     name: "Genericbrand Mover".to_string(),
    //     parameters: BTreeMap::from([
    //         ("brightness".to_string(), "uint8".to_string()),
    //         ("red".to_string(), "uint8".to_string()),
    //         ("green".to_string(), "uint8".to_string()),
    //         ("blue".to_string(), "uint8".to_string()),
    //         ("pan".to_string(), "uint16".to_string()),
    //         ("tilt".to_string(), "uint16".to_string()),
    //         ("gobo_static".to_string(), "1-10: circle 11-20: star 21-30: rectange 31-255: off".to_string()),
    //         ("gobo_rotating".to_string(), "1-10: clouds 11-20: lines 21-30: tree 31-255: off".to_string()),
    //         ("gobo_speed".to_string(), "uint8".to_string()),
    //         ("strobe".to_string(), "0-100: off 101-255: speed".to_string()),
    //         ("focus".to_string(), "uint8".to_string()),
    //         ("zoom".to_string(), "uint8".to_string()),
    //         ("mode".to_string(), "0-10: ok 11-14: reset 15-19: brtdown 20-24: brtup 25-29: park".to_string()),
    //     ])
    // });

    // templates.push(fixture_template::FixtureTemplate {
    //     name: "Genericbrand LED Wash Mover".to_string(),
    //     parameters: BTreeMap::from([
    //         ("red".to_string(), "uint8".to_string()),
    //         ("green".to_string(), "uint8".to_string()),
    //         ("blue".to_string(), "uint8".to_string()),
    //         ("coldwhite".to_string(), "uint8".to_string()),
    //         ("warmwhite".to_string(), "uint8".to_string()),
    //         ("pan".to_string(), "uint16".to_string()),
    //         ("tilt".to_string(), "uint16".to_string()),
    //         ("focus".to_string(), "uint8".to_string()),
    //     ])
    // });

    // templates.push(fixture_template::FixtureTemplate {
    //     name: "Genericbrand Triple LED".to_string(),
    //     parameters: BTreeMap::from([
    //         ("red_1".to_string(), "uint8".to_string()),
    //         ("green_1".to_string(), "uint8".to_string()),
    //         ("blue_1".to_string(), "uint8".to_string()),
    //         ("red_2".to_string(), "uint8".to_string()),
    //         ("green_2".to_string(), "uint8".to_string()),
    //         ("blue_2".to_string(), "uint8".to_string()),
    //         ("red_3".to_string(), "uint8".to_string()),
    //         ("green_3".to_string(), "uint8".to_string()),
    //         ("blue_3".to_string(), "uint8".to_string()),
    //     ])
    // });

    // println!("{:#?}", templates);

    // let context = zmq::Context::new();
    // let server = context.socket(zmq::REP).unwrap();
    // server.bind("tcp://*:5555").unwrap();

    // println!("ZeroMQ connection started");

    // let json_str = json!({
    //     // Fixture templates, i.e. fixtures that _can_ be added to the show
    //     "templates": templates,
    //     // Fixtures, i.e. fixtures that are already in the show. (TODO each fixture should correspond to one template)
    //     "fixtures": vec![fixture1, fixture2]
    // }).to_string();

    // loop {
    //     let string = server.recv_string(0).unwrap().unwrap();
    //     println!("Received request: {}", string);

    //     thread::sleep(time::Duration::from_millis(100));

    //     server.send(json_str.as_str(), 0).unwrap();
    // }

    let input1 = smallvec![ViewValue::F64(100.0)]; 
    let input2 = smallvec![ViewValue::F64(100.0), ViewValue::I64(1000), ViewValue::F64(2000.0)]; 


    {
        println!("Node processing test 1");
        let input = NodeDataset{ packets: smallvec![Some(input1.clone())] };
        println!(" Input:  {:?}", input);
        
        let defn = effects::nodes::math::log();
        let node = defn.build("log");

        // node.deprocessor(&input, smallvec![]);
        let null_parameters = smallvec![];
        let output = (node.definition.processor)(&input, &null_parameters, 1);
        println!(" Output: {:?}", output);
    }

    {
        println!("Node processing test 2");
        let input = NodeDataset{ packets: smallvec![Some(input1), Some(input2)] };;
        println!(" Input:  {:?}", input);
        
        let defn = effects::nodes::math::log();
        let node = defn.build("log");

        // node.deprocessor(&input, smallvec![]);
        let null_parameters = smallvec![];
        let output = (node.definition.processor)(&input, &null_parameters, 1);
        println!(" Output: {:?}", output);
    }

    {
        println!("Node processing test 3");
        let random = effects::nodes::math::random();
        let log = effects::nodes::math::log();

        // let node1 = random.build("random1");
        // let node2 = log.build("log1");
        // let node3 = random.build("random2");
        // let node4 = log.build("log2");

        let mut graph = effects::graph::EffectGraph::new();
        let random1 = graph.add_node(random.build("random1"));
        let log1 = graph.add_node(log.build("log1"));
        let random2 = graph.add_node(random.build("random2"));
        let log2 = graph.add_node(log.build("log2"));

        random1.write().unwrap().outputs.push(smallvec![log1.clone()]);
        log1.write().unwrap().inputs.push(Some(random1.clone()));
        log2.write().unwrap().outputs.push(smallvec![log1.clone()]);
        random2.write().unwrap().outputs.push(smallvec![log2.clone()]);
        log2.write().unwrap().inputs.push(Some(random2.clone()));
        log1.write().unwrap().inputs.push(Some(log2.clone()));

        graph.process();
    }
}
