//! Lighting fixtures
//!
//! A fixture is defined by different objects:
//! - A [FixtureTemplate], containing information about the fixture's definition and generic
//!   information. This contains [ParameterType]s.
//! - A [Fixture], tied to one [FixtureTemplate], which describes the instantiation of a
//!   a template inside a [Show]. Each fixture contains the [ParameterValue]s corresponding
//!   one-to-one to the template's parameter types.

pub mod common_fixtures;
pub mod fixture;
pub mod fixture_template;
pub mod library;
pub mod patches;
pub mod selection;

pub use fixture::*;
pub use fixture_template::*;
pub use library::*;
pub use patches::*;
pub use selection::*;

use crate::parameters::{ParameterType, ParameterValue};
use crate::show::Show;
