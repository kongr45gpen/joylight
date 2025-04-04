//! A fixture library where fixture templates can be fetched from.

use std::sync::LazyLock;

use super::common_fixtures::*;
use super::FixtureTemplate;

/// A fixture library contains various fixture templates that users can browse through, and can be
/// potentially converted to fixtures.
pub struct Library {
    templates: Vec<FixtureTemplate>,
}

impl Default for Library {
    fn default() -> Self {
        Self {
            templates: vec![
                DIMMER.clone(),
                TRIPLE_DIMMER.clone(),
                RGB.clone(),
                RGB_DIMMER.clone(),
            ],
        }
    }
}

impl Library {
    /// Get a fixture template by index
    pub fn get_fixture_template(&self, index: usize) -> Option<&FixtureTemplate> {
        self.templates.get(index)
    }

    /// Get all fixture templates
    pub fn get_all_fixture_templates(&self) -> &[FixtureTemplate] {
        &self.templates
    }
}

/// A prefilled library
const LIBRARY: LazyLock<Library> = LazyLock::new(|| Library::default());
