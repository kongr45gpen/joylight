use crate::effects::io::{NodeDataPacket, NodeDataset};
use crate::effects::node::{EffectNodeDefinition, NodeType};
use crate::parameter::parameter_view::ViewValue;
use smallvec::{smallvec, SmallVec};
use std::cell::OnceCell;
use rand::Rng;

pub fn log() -> EffectNodeDefinition {
    EffectNodeDefinition {
        name: "Log".to_string(),
        help: Some("Base 10 logarithm".to_string()),
        parameters: vec![],
        node_type: NodeType::Processing,
        processor: |inputs, _| {
            inputs.map(|input| {
                Ok(NodeDataPacket(
                    input
                        .process_as_floats()?
                        .into_iter()
                        .map(|value| ViewValue::F64(value.log10()))
                        .collect(),
                ))
            })
        },
    }
}

pub fn random() -> EffectNodeDefinition {
    EffectNodeDefinition {
        name: "Random".to_string(),
        help: Some
            ("Generate random numbers".to_string()),
        parameters: vec![],
        node_type: NodeType::Processing,
        processor: |_, _| {
            Ok(NodeDataset::new_single(
                NodeDataPacket(
                    smallvec![
                        ViewValue::F64(rand::rng().random_range(0.0..1.0)),
                    ]
                )
            ))
        },
    }
}

// const log: OnceCell<EffectNodeDefinition> = OnceCell::new();
