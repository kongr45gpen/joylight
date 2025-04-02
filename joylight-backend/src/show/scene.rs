//! Scenes are a way to store collections of parameter values for current or later use.

use std::collections::HashMap;

use anyhow::anyhow;
use log::error;
use uuid::Uuid;

use super::{BlendingMode, Layer};
use crate::fixtures::{FixtureRef, fixture};
use crate::parameters::{ParameterValue, ParameterView, ViewValuePacket};
use crate::utils::{SmartRef, WithUuid};

/// Used as a key for [SceneValue]s
#[derive(Clone, Hash, PartialEq, Eq)]
struct FixtureParameterPair {
    pub fixture: FixtureRef,
    pub parameter: usize,
}

/// A view value and associated view for a parameter change, as stored in a scene
pub struct SceneValue {
    pub value: ViewValuePacket,
    pub view: Box<dyn ParameterView>,
    pub(self) up_to_date: bool,
}

/// A scene is a collection of parameter values for a set of fixtures. The scene maps specific
/// fixtures and parameters to values. In that sense, it cannot be used for more dynamic assignments,
/// e.g. setting the brightness or color of a generic [crate::fixtures::Selection].
pub struct Scene {
    layer: SmartRef<Layer>,
    uuid: Uuid,
    parameters: HashMap<FixtureParameterPair, SceneValue>,
}

impl Scene {
    /// Sets a parameter value for a fixture
    ///
    /// This does not set the fixture value immediately, neither does it add it to the fixture for later update.
    /// This is handled by [Scene::add_parameter_updates].
    pub fn set_parameter(
        &mut self,
        fixture: FixtureRef,
        parameter: usize,
        value: ViewValuePacket,
        view: Box<dyn ParameterView>,
    ) {
        let pair = FixtureParameterPair { fixture, parameter };
        self.parameters.insert(
            pair,
            SceneValue {
                value,
                view,
                up_to_date: false,
            },
        );
    }

    /// Removes a parameter from this scene
    ///
    /// This also removes the parameter update from the fixture immediately, if it exists.
    pub fn clear_parameter(&mut self, fixture: FixtureRef, parameter: usize) {
        self.layer.remove_value(&fixture, parameter);

        let pair = FixtureParameterPair { fixture, parameter };
        self.parameters.remove(&pair);
    }

    /// Removes all parameters from this scene
    ///
    /// This also removes all parameter updates from the fixture immediately.
    pub fn clear(&mut self) {
        self.layer.clear_values();
        self.parameters.clear();
    }

    /// Gets the parameter value for a fixture
    pub fn get_parameter(&self, fixture: FixtureRef, parameter: usize) -> Option<&SceneValue> {
        self.parameters.get(&FixtureParameterPair { fixture, parameter })
    }

    /// Gets a reference to the layer associated with this scene and this scene only
    pub fn get_layer(&self) -> &SmartRef<Layer> {
        &self.layer
    }

    /// Prepare the proper parameter updates of this scene's layer for each fixture.
    ///
    /// Note that this does not perform the final calculation of a parameter's final value. This needs
    /// to be done during rendering.
    pub fn add_parameter_updates(&mut self) {
        for (key, set) in self.parameters.iter_mut() {
            if !set.up_to_date {
                match set.view.to_value(&set.value) {
                    Ok(value) => {
                        self.layer.set_value(&key.fixture, key.parameter, &value);
                    }
                    Err(e) => {
                        error!(
                            "Could not convert view to parameter for fixture {{ {} }}, parameter {}: {:?}",
                            key.fixture.uuid(),
                            key.parameter,
                            set.value
                        );
                    }
                }
            }
        }
    }
}

impl Default for Scene {
    fn default() -> Self {
        Self {
            layer: SmartRef::new_from_move(Layer::new("Scene", BlendingMode::Add, 0)),
            uuid: Uuid::new_v4(),
            parameters: HashMap::new(),
        }
    }
}

impl WithUuid for Scene {
    fn uuid(&self) -> Uuid {
        self.uuid
    }
}
