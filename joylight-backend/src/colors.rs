//! Color management utilities

use ndarray::prelude::*;
use ndarray::arr2;
use ndarray_linalg::Solve;
use ndarray_linalg::Inverse;
use ndarray_linalg::LeastSquaresSvd;
use std::fmt;

/// A simple representation of an RGB color
///
/// TODO: Use a more advanced color representation based on another crate, perhaps
#[derive(Clone)]
pub struct RGBTuple(pub [f64; 3]);

impl fmt::Debug for RGBTuple {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "RGB [{}, {}, {}]",
            self.0[0], self.0[1], self.0[2]
        ))
    }
}

pub fn red() -> RGBTuple {
    RGBTuple([1.0, 0.0, 0.0])
}

pub fn green() -> RGBTuple {
    RGBTuple([0.0, 1.0, 0.0])
}

pub fn blue() -> RGBTuple {
    RGBTuple([0.0, 0.0, 1.0])
}

pub fn white() -> RGBTuple {
    RGBTuple([1.0, 1.0, 1.0])
}

pub fn rgb_base() -> Vec<RGBTuple> {
    vec![red(), green(), blue()]
}

/// A list of non-component based color models
#[derive(Clone, Debug)]
pub enum ColorModel {
    /// Hue-Saturation-Value
    HSV,
    /// Hue-Saturation-Lightness
    HSL,
}

fn change_base_3d(
    from: Array2<f64>,
    to: Array2<f64>,
    color: Array1<f64>,
) -> Result<Array1<f64>, ()> {
    Ok(to.t().inv().map_err(|_| ())?.dot(&from).dot(&color))
}

fn best_fit(
    from: Array2<f64>,
    to: Array2<f64>,
    color: Array1<f64>,
) -> Result<Array1<f64>, ()> {
    let ls = to.t().least_squares(&color).map_err(|_| ())?;
    Ok(ls.solution)
}

pub fn change_base(
    from: Vec<RGBTuple>,
    to: Vec<RGBTuple>,
    color: Vec<f64>,
) -> Result<Vec<f64>,()> {
    if from.len() < 2 || to.len() < 2 {
        return Err(());
    }

    let a1 = Array2::from_shape_vec(
        (from.len(), 3),
        from.iter().flat_map(|RGBTuple(rgb)| rgb.iter().map(|&x| x as f64)).collect(),
    ).map_err(|_| ())?;

    let a2 = Array2::from_shape_vec(
        (to.len(), 3),
        to.iter().flat_map(|RGBTuple(rgb)| rgb.iter().map(|&x| x as f64)).collect(),
    ).map_err(|_| ())?;

    let b = Array::from_vec(color);

    println!("A1= {:#?}\nΑ2 = {:#?}\nb= {:#?}", a1, a2, b);

    if from.len() == 3 && to.len() == 3 {
        let attempt = change_base_3d(a1, a2, b);

        if let Ok(result) = attempt {
            return Ok(result.to_vec());
        }
    } else {
        let attempt = best_fit(a1, a2, b);

        if let Ok(result) = attempt {
            return Ok(result.to_vec());
        }
    }

    return Err(());


}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn one_to_one_base_change() {
        let color = vec![0.25, 0.50, 0.90];

        let result = change_base(rgb_base(), rgb_base(), color.clone()).unwrap();

        assert_eq!(color, result);
    }

    #[test]
    fn to_base_reorder() {
        let from_base = vec![red(), green(), blue()];
        let to_base = vec![green(), blue(), red()];

        let color = vec![0.25, 0.50, 0.90];

        let result = change_base(from_base, to_base, color.clone()).unwrap();

        assert_eq!(vec![0.50, 0.90, 0.25], result);
    }

    #[test]
    fn from_base_reorder() {
        let from_base = vec![blue(), green(), red()];
        let to_base = vec![green(), blue(), red()];

        let color = vec![0.90, 0.50, 0.25];

        let result = change_base(from_base, to_base, color.clone()).unwrap();

        assert_eq!(vec![0.50, 0.90, 0.25], result);
    }

    #[test]
    fn rgb_to_rmc() {
        let from_base: Vec<RGBTuple> = rgb_base();
        let to_base: Vec<RGBTuple> = vec![
            RGBTuple([1.0, 0.0, 0.0]),
            RGBTuple([0.0, 1.0, 1.0]),
            RGBTuple([1.0, 0.0, 1.0]),
        ];

        let color = vec![0.5, 0.5, 1.0];
        // let color = vec![0.0, 1.0, 1.0];

        let result = change_base(from_base, to_base, color).unwrap();

        assert_eq!(vec![0.0, 0.5, 0.5], result);
    }

    #[test]
    #[ignore="not implemented"]
    fn rgb_to_rg() {
        let from_base = rgb_base();
        let to_base = vec![red(), green()];
        let color = vec![0.5, 0.0, 0.0];

        let result = change_base(from_base, to_base, color.clone());

        // assert_eq!(vec![0.5], result.unwrap());
        assert_eq!(Err(()), result);
    }

    #[test]
    fn inadequate_base() {
        let from_base = rgb_base();
        let to_base = vec![red(), green(), green()];
        let color = vec![0.25, 0.50, 0.90];

        let result = change_base(from_base, to_base, color.clone());

        assert_eq!(Err(()), result);
    }

    #[test]
    fn multiple_rgb_solutions() {
        let from_base = rgb_base();
        let to_base = vec![
            RGBTuple([1.0, 0.0, 0.0]),
            RGBTuple([1.0, 0.0, 0.0]),
            RGBTuple([1.0, 0.0, 0.0]),
        ];
        let color = vec![1.0, 0.0, 0.0];

        let result = change_base(from_base, to_base, color.clone());

        // assert_eq!(vec![0.33, 0.33, 0.33], result);
        assert_eq!(Err(()), result);
    }

    #[test]
    #[should_panic]
    fn color_incompatible_with_base() {
        let from_base = rgb_base();
        let to_base = rgb_base();
        let color = vec![0.25, 0.50];

        let result = change_base(from_base, to_base, color.clone());

        assert_eq!(Err(()), result);
    }

    #[test]
    #[ignore="not implemented"]
    fn rgb_to_rgbw() {
        let from_base = rgb_base();
        let to_base = vec![red(), green(), blue(), white()];
        let color = vec![0.0, 0.0, 1.0];

        let result = change_base(from_base, to_base, color.clone()).unwrap();

        assert_eq!(vec![0.0, 0.0, 0.0, 1.0], result);
    }
}