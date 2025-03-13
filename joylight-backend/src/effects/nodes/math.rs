use crate::effects::io::{packet_to_f64, NodeDataset};
use crate::effects::node::{EffectNodeDefinition, NodeType};
use crate::parameter::parameter_view::{ViewValue, ViewValuePacket};
use rand::Rng;
use smallvec::{smallvec, SmallVec};
use std::cell::OnceCell;

pub fn log() -> EffectNodeDefinition {
    EffectNodeDefinition {
        name: "Log".to_string(),
        help: Some("Base 10 logarithm".to_string()),
        parameters: vec![],
        node_type: NodeType::Processing,
        processor: |inputs, _, _| {
            inputs.map_values(|packet| {
                packet_to_f64(packet)?
                    .into_iter()
                    .map(|value| Ok(ViewValue::F64(value.log10())))
                    .collect()
            })
        },
    }
}

pub fn random() -> EffectNodeDefinition {
    EffectNodeDefinition {
        name: "Random".to_string(),
        help: Some("Generate random numbers".to_string()),
        parameters: vec![],
        node_type: NodeType::Input,
        processor: |_, _, n| {
            Ok(NodeDataset::new_from_generator(n, || {
                smallvec![ViewValue::F64(rand::rng().random_range(0.0..1.0))]
            }))
        },
    }
}

// const log: OnceCell<EffectNodeDefinition> = OnceCell::new();
