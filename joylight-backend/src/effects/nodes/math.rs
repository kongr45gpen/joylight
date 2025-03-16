use crate::effects::io::{packet_to_f64, NodeDataset};
use crate::effects::node::{EffectNodeDefinition, NodeType};
use crate::parameter::parameter_view::{ViewValue, ViewValuePacket};
use anyhow::{anyhow, Result};
use rand::Rng;
use serde::de;
use smallvec::{smallvec, SmallVec};
use std::cell::OnceCell;
use std::time::{Instant, SystemTime};
use log::debug;

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
        processor: Box::new(|inputs, _, _| universal_math(inputs, f64::abs)),
    }
}

pub fn log() -> EffectNodeDefinition {
    EffectNodeDefinition {
        name: "Log10".to_string(),
        help: Some("Base 10 logarithm".to_string()),
        parameters: vec![],
        node_type: NodeType::Processing,
        processor: Box::new(|inputs, _, _| universal_math(inputs, f64::log10)),
    }
}

pub fn sin() -> EffectNodeDefinition {
    EffectNodeDefinition {
        name: "Sin".to_string(),
        help: Some("Sine function".to_string()),
        parameters: vec![],
        node_type: NodeType::Processing,
        processor: Box::new(|inputs, _, _| universal_math(inputs, f64::sin)),
    }
}

pub fn random() -> EffectNodeDefinition {
    EffectNodeDefinition {
        name: "Random".to_string(),
        help: Some("Generate random numbers".to_string()),
        parameters: vec![],
        node_type: NodeType::Input,
        processor: Box::new(|_, _, n| {
            Ok(NodeDataset::new_from_generator(n, || {
                smallvec![ViewValue::F64(rand::rng().random_range(0.0..1.0))]
            }))
        }),
    }
}

pub fn clock(instant: Instant) -> EffectNodeDefinition {
    EffectNodeDefinition {
        name: "Clock".to_string(),
        help: Some("The system clock in float seconds".to_string()),
        parameters: vec![],
        node_type: NodeType::Input,
        processor: Box::new(move |_, _, _| {
            let time = instant
                .elapsed()
                .as_secs_f64();

            debug!("Clock: {}", time);

            Ok(NodeDataset::new_single(smallvec![ViewValue::F64(time)]))
        }),
    }
}
