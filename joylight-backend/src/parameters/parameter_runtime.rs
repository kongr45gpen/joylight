use std::collections::HashMap;
use std::time::Instant;

use log::warn;
use uuid::Uuid;

use super::{ParameterValue, ParameterValueDescription, ViewValue, parameter_type};
use crate::show::{Layer, make_decision};
use crate::utils::{SmartRef, WithUuid};

#[derive(Clone, Debug)]
pub struct ParameterUpdate {
    /// The value to set the parameter to
    pub value: ParameterValue,
    /// The layer responsible for this update
    pub layer: SmartRef<Layer>,
    /// The time at which this value was updated
    pub updated: Instant,
}

/// A representation of the current, real-time value of the parameter.
#[derive(Clone, Debug)]
pub struct ParameterRuntime {
    /// The description as used during run-time. We store this in addition to the parameter template,
    /// in case there are updates to this value in the show.
    pub description: ParameterValueDescription,
    /// The intermal representation of the parameter's value in the parameter's model.
    pub value: ParameterValue,
    /// The parameter's view representation. This is stored in addition to [ParameterRuntime::value], since
    /// there may not be a 1:1 reverse mapping from a view to a value.
    pub view_values: Vec<ViewValue>,
    /// Any updates to the parameter value that are queued for the next refresh.
    ///
    /// Whenever a fixture is added or removed from a layer, and whenever a layer is added or removed,
    /// it should update this value.
    pub(crate) updates: HashMap<Uuid, ParameterUpdate>,
    /// Whether this parameter's value is up-to-date.
    ///
    /// Reset this whenever updating a value in [ParameterRuntime::updates]
    pub up_to_date: bool,
}

impl ParameterRuntime {
    /// Create a new parameter runtime
    pub fn new(description: ParameterValueDescription, value: ParameterValue) -> Self {
        Self {
            description,
            value,
            //TODO
            view_values: Vec::new(),
            updates: HashMap::new(),
            up_to_date: false,
        }
    }

    pub fn new_from_type(parameter_type: &parameter_type::ParameterType) -> Self {
        Self::new(parameter_type.description.clone(), parameter_type.default_value.clone())
    }

    pub(crate) fn set_value_from_layer(&mut self, layer: SmartRef<Layer>, value: ParameterValue) {
        self.updates.insert(
            layer.uuid(),
            ParameterUpdate {
                value,
                layer,
                updated: Instant::now(),
            },
        );

        self.up_to_date = false;
    }

    pub(crate) fn remove_layer(&mut self, layer: SmartRef<Layer>) {
        self.updates.remove(&layer.uuid());
        self.up_to_date = false;
    }

    pub(crate) fn remove_all_layers(&mut self) {
        self.updates.clear();
        self.up_to_date = false;
    }
}
