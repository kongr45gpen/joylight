mod fixture;
mod parameter;

use std::boxed::Box;

use std::net::UdpSocket;

use serde_json::json;
use std::{thread, time};

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

    println!("{:#?}", fixture1);
    println!("{:#?}", fixture2);

    let socket = UdpSocket::bind("127.0.0.1:7096").expect("Couldn't bind to address");


    while true {
        let mut buf = [0; 10];
        let (number_of_bytes, src_addr) = socket.recv_from(&mut buf)
                                                .expect("Didn't receive data");
    
        let mut json_buf = [0; 255];
        let json_str = json!([fixture1, fixture2]).to_string();
        for (i, c) in json_str.chars().enumerate() {
            json_buf[i] = c as u8;
        }
    
        println!("Received {} bytes from {:?}", number_of_bytes, src_addr);
        socket.send_to(&json_buf, src_addr).expect("Couldn't send data");
    }
}
