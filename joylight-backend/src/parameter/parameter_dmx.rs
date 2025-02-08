//! Encoding descriptions for DMX parameters

use std::{fmt::Debug, ops::{Sub, Mul}, cmp};
use serde::{Deserialize, Serialize};
use dyn_clone::DynClone;

use crate::parameter::parameter_value::ParameterValue;

/// Maps a number from an input to an output range
/// 
/// TODO: Maybe it's just easier to split this into ints and floats
fn map_number<T: Sub<Output=T> + PartialOrd + Into<f64> + Mul<f64, Output=f64> + Clone>(input_min: T, input_max: T, output_min: u64, output_max: u64, input: T) -> u64 {
    if input <= input_min {
        return output_min;
    } else if input >= input_max {
        return output_max;
    }

    let input_range: f64 = input_max.into() - input_min.clone().into();
    let output_range = cmp::max(output_max.checked_sub(output_min).unwrap_or(1u64), 1u64) as f64;

    let input_diff = input - input_min;
    let factor = output_range / input_range;

    return output_min + (input_diff * factor).round() as u64;
}

/// A [ParameterEncoder] transforms a [ParameterValue] into something that can be read by an external interface,
/// such as DMX, MQTT or others.
pub trait ParameterEncoder: DynClone + Debug {
    fn encode(&self, value: ParameterValue) -> Result<Vec<u8>, ()>;
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Endianness {
    /// DMX big-endian `[coarse, fine]`
    Big,
    /// DMX little-endian `[fine, coarse]`
    Little,
    /// Collated big-endian (combines parameters, e.g. `[coarse1, coarse2, fine1, fine2]`)
    BigCollated,
    /// Collated little-endian (combines parameters, e.g. `[fine1, fine2, coarse1, coarse2]`)
    LittleCollated,
}

/// A transformer from a [crate::parameter_value::Number] to a DMX value
#[derive(Clone, Debug)]
pub struct DMXMappingTransformer {
    /// This input value corresponds to the value `0` in DMX.
    pub input_min: f64,
    /// This input value corresponds to the value $256^{\text{size}} - 1$ in DMX.
    pub input_max: f64,
    /// The number of DMX parameters corresponding to each input value.
    /// This is useful for parameters larger than 1 byte, usually "fine control".
    pub size: u8,
    /// Defines the order of the bytes in each value.
    pub endianness: Endianness,
}

impl ParameterEncoder for DMXMappingTransformer {
    fn encode(&self, value: ParameterValue) -> Result<Vec<u8>, ()> {
        let ParameterValue::Number(numbers) = value else {
            return Err(());
        };

        let stride = self.size as usize;
        let count = numbers.len();

        if stride > 8 {
            return Err(());
        }

        let mut result = vec![0; count * stride];

        for (i, number) in numbers.iter().enumerate() {
            let dmx_max = 256u64.pow(self.size as u32) - 1;

            let dmx_value = map_number(self.input_min, self.input_max, 0, dmx_max, *number);

            // Convert the number to a byte array
            let bytes = dmx_value.to_le_bytes();

            // for  in 0..self.size {
            match self.endianness {
                Endianness::Big => {
                    for j in 0..stride {
                        result[i * stride + j] = bytes[stride - j - 1];
                    }
                },
                Endianness::Little => {
                    for j in 0..stride {
                        result[i * stride + j] = bytes[j];
                    }
                },
                Endianness::BigCollated => {
                    for j in 0..stride {
                        result[i + j * count] = bytes[stride - j - 1];
                    }
                },
                Endianness::LittleCollated => {
                    for j in 0..stride {
                        result[i + j * count] = bytes[j];
                    }
                },
            }
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn size_one_transformer() {
        let transformer = DMXMappingTransformer {
            input_min: 0.0,
            input_max: 100.0,
            size: 1,
            endianness: Endianness::Big
        };

        {
            let single_value = ParameterValue::Number(vec![99.0]);
            let result = transformer.encode(single_value).unwrap();

            assert_eq!(result, vec![252]);
        }

        {
            let multiple_value = ParameterValue::Number(vec![0.0, 45.0, 100.0]);
            let result = transformer.encode(multiple_value).unwrap();

            assert_eq!(result, vec![0, 115, 255]);
        }
    }

    #[test]
    fn big_endian_transformer() {
        let transformer = DMXMappingTransformer {
            input_min: 0.0,
            input_max: 100.0,
            size: 2,
            endianness: Endianness::Big
        };

        let value = ParameterValue::Number(vec![55.0, 99.611]);
        let result = transformer.encode(value).unwrap();

        assert_eq!(result, vec![0x8C, 0xCC, 0xFF, 0x00]);
    }

    #[test]
    fn little_endian_transformer() {
        let transformer = DMXMappingTransformer {
            input_min: 0.0,
            input_max: 100.0,
            size: 2,
            endianness: Endianness::Little
        };

        let value = ParameterValue::Number(vec![55.0, 99.611]);
        let result = transformer.encode(value).unwrap();

        assert_eq!(result, vec![0xCC, 0x8C, 0x00, 0xFF]);
    }

    #[test]
    fn big_collated_endian_transformer() {
        let transformer = DMXMappingTransformer {
            input_min: 0.0,
            input_max: 100.0,
            size: 2,
            endianness: Endianness::BigCollated
        };

        let value = ParameterValue::Number(vec![55.0, 99.611, 98.045]);
        let result = transformer.encode(value).unwrap();

        assert_eq!(result, vec![0x8C, 0xFF, 0xFA, 0xCC, 0x00, 0xFE]);
    }

    #[test]
    fn little_collated_endian_transformer() {
        let transformer = DMXMappingTransformer {
            input_min: 0.0,
            input_max: 100.0,
            size: 2,
            endianness: Endianness::LittleCollated
        };

        let value = ParameterValue::Number(vec![55.0, 99.611, 98.045]);
        let result = transformer.encode(value).unwrap();

        assert_eq!(result, vec![0xCC, 0x00, 0xFE, 0x8C, 0xFF, 0xFA]);
    }

    #[test]
    fn high_density_transformer() {
        let transformer = DMXMappingTransformer {
            input_min: 0.0,
            input_max: 140.0,
            size: 3,
            endianness: Endianness::Little
        };

        let value = ParameterValue::Number(vec![55.0, 57.0, 139.0]);
        let result = transformer.encode(value).unwrap();

        assert_eq!(result, vec![0x49, 0x92, 0x64, 0x83, 0x3A, 0x68, 0xE2, 0x2B, 0xFE]);
    }

    #[test]
    fn transformer_with_out_of_bounds_inputs() {
        let transformer = DMXMappingTransformer {
            input_min: 0.0,
            input_max: 100.0,
            size: 1,
            endianness: Endianness::Big
        };

        {
            let single_value = ParameterValue::Number(vec![-1.0]);
            let result = transformer.encode(single_value).unwrap();

            assert_eq!(result, vec![0]);
        }

        {
            let single_value = ParameterValue::Number(vec![101.0]);
            let result = transformer.encode(single_value).unwrap();

            assert_eq!(result, vec![255]);
        }
    }

    #[test]
    fn incorrectly_defined_transformers_dont_panic() {
        let transformer1 = DMXMappingTransformer {
            input_min: 100.0,
            input_max: 0.0,
            size: 1,
            endianness: Endianness::Big
        };

        let _ = transformer1.encode(ParameterValue::Number(vec![55.0]));

        let transformer2 = DMXMappingTransformer {
            input_min: 100.0,
            input_max: 100.0,
            size: 1,
            endianness: Endianness::Big
        };

        let _ = transformer2.encode(ParameterValue::Number(vec![55.0]));

        let transformer3 = DMXMappingTransformer {
            input_min: 100.0,
            input_max: 100.0,
            size: 0,
            endianness: Endianness::Big
        };

        let _ = transformer3.encode(ParameterValue::Number(vec![0.0]));
    }
}