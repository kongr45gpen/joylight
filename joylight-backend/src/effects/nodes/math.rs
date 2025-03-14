use crate::effects::io::{packet_to_f64, NodeDataset};
use crate::effects::node::{EffectNodeDefinition, NodeType};
use crate::parameter::parameter_view::{ViewValue, ViewValuePacket};
use rand::Rng;
use smallvec::{smallvec, SmallVec};
use std::cell::OnceCell;
use anyhow::{anyhow, Result};

fn universal_math<F>(inputs: &NodeDataset, op: F) -> Result<NodeDataset>
where
    F: Fn(f64) -> f64 + 'static,
{
    inputs.map_values(|packet| {
        packet_to_f64(packet)?
            .iter()
            .map(|value| Ok(ViewValue::F64(op(*value))))
            .collect()
    })
}

pub fn abs() -> EffectNodeDefinition {
    EffectNodeDefinition {
        name: "Abs".to_string(),
        help: Some("Absolute Value".to_string()),
        parameters: vec![],
        node_type: NodeType::Processing,
        processor: |inputs, _, _| {
            universal_math(inputs, f64::abs)
        },
    }
}

pub fn log() -> EffectNodeDefinition {
    EffectNodeDefinition {
        name: "Log10".to_string(),
        help: Some("Base 10 logarithm".to_string()),
        parameters: vec![],
        node_type: NodeType::Processing,
        processor: |inputs, _, _| {
            universal_math(inputs, f64::log10)
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
