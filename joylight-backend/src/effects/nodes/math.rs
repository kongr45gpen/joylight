use crate::effects::io::{NodeDataPacket, NodeDataset};
use crate::effects::node::{EffectNodeDefinition, NodeType};
use crate::parameter::parameter_view::ViewValue;
use smallvec::SmallVec;
use std::cell::OnceCell;

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

// const log: OnceCell<EffectNodeDefinition> = OnceCell::new();
