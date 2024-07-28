use std::fmt::Debug;
use serde::{Deserialize, Serialize};

pub trait DMXParameter : Debug + Serialize {
    fn get_size(&self) -> u8;
    fn get_value(&self) -> Vec<u8>;
}

#[derive(Debug, Serialize)]
pub struct Brightness {
    pub value: f32
}

impl DMXParameter for Brightness {
    fn get_size(&self) -> u8 { 1 }

    fn get_value(&self) -> Vec<u8> { vec![(self.value * 255.0) as u8] }
}

#[derive(Debug, Serialize)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32
}

impl DMXParameter for Color {
    fn get_size(&self) -> u8 { 3 }

    fn get_value(&self) -> Vec<u8> {
        vec![
            (self.r * 255.0) as u8,
            (self.g * 255.0) as u8,
            (self.b * 255.0) as u8
        ]
    }
}

#[derive(Debug, Serialize)]
struct Rotation {
    pan: f32,
    tilt: f32
}

impl DMXParameter for Rotation {
    fn get_size(&self) -> u8 { 2 }

    fn get_value(&self) -> Vec<u8> {
        vec![
            (self.pan * 255.0) as u8,
            (self.tilt * 255.0) as u8
        ]
    }
}

#[derive(Debug, Serialize)]
pub enum DMXParameterType {
    Brightness(Brightness),
    Color(Color),
    Rotation(Rotation)
}