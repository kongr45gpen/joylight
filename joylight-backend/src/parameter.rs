//! This create defines all parameter types that belong in a fixture.
//! 
//! The main entities defined here are parameter **types**. They do not contain value, but only
//! explain the kind of value that will be stored by the fixture.
//! 
//! This file contains two types of entities: [ParameterTypes](ParameterType) and [ParameterRecords](ParameterRecord).
//! A [ParameterType] describes the type, size, format of a parameter, but does not contain its current value. It is
//! associated with fixture templates.
//! A [ParameterRecord] contains the current value of a parameter. It is associated with fixture instances.
//! 
//! The structure of the ParameterRecord defines the internal representation (i.e. model) of the value in the backend.
//!
//! The fields and methods of ParameterType define how the value is converted into DMX or other formats (e.g. how many
//! bytes are used, color spaces etc). Note that there is no 1-1 mapping between a DMX parameter and our ParameterType.

use std::fmt::Debug;
use std::any::Any;
use serde::{Deserialize, Serialize};

/// Byteorder enum
#[derive(Debug, Serialize)]
pub enum ByteOrder {
    BigEndian,
    LittleEndian,
}

// TODO: Remove 'static requirement
// TODO: Move from dyn Any to dyn ParameterRecord
// TODO: Do not require clone?
pub trait ParameterRecord : Debug + Clone + Default + Sized {
    fn downcast_ref(record: &dyn Any) -> Self where Self: 'static{
        record.downcast_ref::<Self>()
            .map(|r| r.clone())
            .unwrap_or_default()
    }
}

/// A parameter type 
pub trait ParameterType : ParameterTypeBase + Debug {
    type Record : ParameterRecord;
}

/// Useful to perform type erasure because Rust doesn't like associated types in traits
pub trait ParameterTypeBase: Debug { 
    fn new_record(&self) -> Box<dyn Any>;
}


impl <T: ParameterType> ParameterTypeBase for T
where <T as ParameterType>::Record: 'static {
    fn new_record(&self) -> Box<dyn Any> {
        Box::new(T::Record::default())
    }
}

/// A DMX parameter can be converted to a contiguous bytearray suitable for the DMX protocol.
pub trait DMXParameter : ParameterTypeBase {
    fn get_size(&self) -> u8;
    fn get_value(&self, record: &dyn Any) -> Vec<u8>;
}

#[derive(Debug, Serialize)]
pub struct Brightness {
    pub size: u8,
    pub endianness: ByteOrder,
}

impl ParameterType for Brightness {
    type Record = f32;
}

impl ParameterRecord for <Brightness as ParameterType>::Record {}

impl DMXParameter for Brightness {
    fn get_size(&self) -> u8 {
        self.size
    }

    fn get_value(&self, record: &dyn Any) -> Vec<u8> {
        let float_value = <Self as ParameterType>::Record::downcast_ref(record);
        let int_value: u64 = (float_value * (((2 << (8 * self.size - 1)) - 1) as f32)) as u64;

        match self.endianness {
            ByteOrder::BigEndian => int_value.to_be_bytes()[8 - self.size as usize..].to_vec(),
            ByteOrder::LittleEndian => int_value.to_le_bytes()[0..self.size as usize].to_vec(),
        }
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct ColorRecord {
    pub color: [f32; 3],
}

#[derive(Debug, Serialize)]
pub enum RGBOrder {
    RGB,
    RBG,
    GRB,
    GBR,
    BRG,
    BGR,
}

#[derive(Debug, Serialize)]
pub struct RGB {
    pub order: RGBOrder,
}

impl ParameterType for RGB {
    type Record = ColorRecord;
}

impl ParameterRecord for ColorRecord {}
impl Default for ColorRecord {
    fn default() -> Self {
        ColorRecord {
            color: [0.0, 0.0, 0.0],
        }
    }
}

impl DMXParameter for RGB {
    fn get_size(&self) -> u8 {
        3
    }

    fn get_value(&self, record: &dyn Any) -> Vec<u8> {
        let colors_float = <Self as ParameterType>::Record::downcast_ref(record);

        let colors_int: [u8; 3] = [
            (colors_float.color[0] * 255.0) as u8,
            (colors_float.color[1] * 255.0) as u8,
            (colors_float.color[2] * 255.0) as u8,
        ];

        let colors_int_reordered = match self.order {
            RGBOrder::RGB => colors_int,
            RGBOrder::RBG => [colors_int[0], colors_int[2], colors_int[1]],
            RGBOrder::GRB => [colors_int[1], colors_int[0], colors_int[2]],
            RGBOrder::GBR => [colors_int[1], colors_int[2], colors_int[0]],
            RGBOrder::BRG => [colors_int[2], colors_int[0], colors_int[1]],
            RGBOrder::BGR => [colors_int[2], colors_int[1], colors_int[0]],
        };

        colors_int_reordered.to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn brightness_parameter_1b() {
        let brightness = Brightness {
            size: 1,
            endianness: ByteOrder::BigEndian,
        };

        let record: f32 = 0.5;

        let value = brightness.get_value(&record);
        assert_eq!(value, vec![127]);
    }

    #[test]
    fn brightness_parameter_2be() {
        let brightness = Brightness {
            size: 2,
            endianness: ByteOrder::BigEndian,
        };

        let record: f32 = 0.75;

        let value = brightness.get_value(&record);
        assert_eq!(value, vec![191, 255]);
    }

    #[test]
    fn brightness_parameter_2le() {
        let brightness = Brightness {
            size: 2,
            endianness: ByteOrder::LittleEndian,
        };

        let record: f32 = 0.75;

        let value = brightness.get_value(&record);
        assert_eq!(value, vec![255, 191]);
    }

    #[test]
    fn color_parameter() {
        let parameter_type = RGB {
            order: RGBOrder::BGR,
        };

        let record = ColorRecord {
            color: [0.393, 0.785, 0.251],
        };

        let value = parameter_type.get_value(&record);
        assert_eq!(value, vec![64, 200, 100]);
    }
}