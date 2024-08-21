//! This create defines all parameter types that belong in a fixture.
//! 
//! The main entities defined here are parameter **types**. They do not contain value, but only
//! explain the kind of value that will be stored by the fixture.

use std::fmt::Debug;
use std::any::Any;
use serde::{Deserialize, Serialize};

/// Byteorder enum
#[derive(Debug, Serialize)]
pub enum ByteOrder {
    BigEndian,
    LittleEndian,
}

/// A parameter type 
pub trait ParameterType : Debug {
    type Record;
}

/// A DMX parameter can be converted to a contiguous bytearray suitable for the DMX protocol.
pub trait DMXParameter : ParameterType + Debug + Serialize {
    fn get_size(&self) -> u8;
    fn get_value(&self, record: &dyn Any) -> Box<[u8]>;
}

#[derive(Debug, Serialize)]
pub struct Brightness {
    pub size: u8,
    pub endianness: ByteOrder,
}

impl ParameterType for Brightness {
    type Record = f32;
}

impl DMXParameter for Brightness {
    fn get_size(&self) -> u8 {
        self.size
    }

    fn get_value(&self, record: &dyn Any) -> Box<[u8]> {
        let float_value = *(record.downcast_ref::<f32>().unwrap_or(&0.0));
        let int_value: u64 = (float_value * (((2 << (8 * self.size - 1)) - 1) as f32)) as u64;

        println!("float_value: {}, int_value: {}", float_value, int_value);

        let bytes = match self.endianness {
            ByteOrder::BigEndian => int_value.to_be_bytes()[8 - self.size as usize..].to_vec(),
            ByteOrder::LittleEndian => int_value.to_le_bytes()[0..self.size as usize].to_vec(),
        };

        bytes.into_boxed_slice()
    }
}
