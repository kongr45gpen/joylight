use std::{fmt::format, sync::{Arc, RwLock}};
use crate::{effects::{io::{packet_to_f64, NodeDataset}, node::{EffectNodeDefinition, NodeType}}, fixture::selection::Selection, parameter::{self, parameter_value}};
use log::*;
use anyhow::anyhow;

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

            let paket = packet_to_f64(&inputs.packets.first().unwrap().as_ref().unwrap())?;
            
            for fixture in selection.fixtures() {
                debug!("iterating over fixture {} for parameter {}",  fixture.read().unwrap().name, named_parameter);

                let mut fxt_write = fixture.write().unwrap();
                let mut param = fxt_write.get_parameter_by_name(named_parameter.as_str())
                    .ok_or_else(|| anyhow!("Parameter {} not found", named_parameter))?;



                *param = parameter_value::ParameterValue::Number(
                    paket.iter().map(|n| *n).collect()
                );
            }

            Ok(Default::default())
        })
    }
}