use std::fmt::format;
use std::sync::{Arc, RwLock};

use anyhow::{anyhow, Context};
use chumsky::debug;
use log::*;

use crate::effects::io::{packet_to_f64, NodeDataset};
use crate::effects::node::{EffectNodeDefinition, NodeType};
use crate::fixtures::Selection;
use crate::parameters::{self, parameter_value};

pub fn output(named_parameter: &str, selection: Arc<RwLock<dyn Selection>>) -> EffectNodeDefinition {
    let named_parameter = named_parameter.to_string();

    EffectNodeDefinition {
        name: format!("Output {}", named_parameter),
        help: Some("Output the value".to_string()),
        parameters: vec![],
        node_type: NodeType::Output,
        processor: Box::new(move |inputs: &NodeDataset, _, _| {
            inputs.check_count(1)?;

            let selection = selection.read().unwrap();

            let paket = packet_to_f64(inputs.packets.first().unwrap())?;

            for fixture in selection.fixtures() {
                debug!(
                    "iterating over fixture {} for parameter {}",
                    fixture.uuid(),
                    named_parameter
                );

                fixture
                    .write(|fxt| {
                        fxt.get_parameter_by_name(named_parameter.as_str())
                            .ok_or_else(|| anyhow!("Parameter {} not found", named_parameter))
                            .and_then(|index| {
                                debug!("Found parameter {:?} corresponding to name {}", index, named_parameter);
                                fxt.set_parameter(
                                    index,
                                    parameter_value::ParameterValue::Number(paket.iter().copied().collect()),
                                )
                            })
                    })
                    .inspect_err(|e| error!("Error setting parameter: {}", e))
                    .context("Setting parameter")?;
            }

            Ok(Default::default())
        }),
    }
}
