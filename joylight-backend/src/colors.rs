use std::fmt;

/// A simple representation of an RGB color
///
/// TODO: Use a more advanced color representation based on another crate, perhaps
#[derive(Clone)]
pub struct RGBTuple(pub [u8; 3]);

impl fmt::Debug for RGBTuple {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("RGB [{}, {}, {}]", self.0[0], self.0[1], self.0[2]))
    }
}

/// A list of non-component based color models
#[derive(Clone, Debug)]
pub enum ColorModel {
    /// Hue-Saturation-Value
    HSV,
    /// Hue-Saturation-Lightness
    HSL,
}
