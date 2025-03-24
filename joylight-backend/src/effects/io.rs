use smallvec::SmallVec;
use std::fmt::Debug;
use anyhow::{anyhow, Context, Result};

use crate::parameter::parameter_view::{ViewValue, ViewValuePacket};

/// A node may have multiple inputs and outputs that may themselves be vectors.
/// A "dataset" represents all the set of [ViewValuePacket]s that are input or output by the node.
#[derive(Debug, Clone, Default)]
pub struct NodeDataset {
    pub packets: SmallVec<[ViewValuePacket; 3]>,
}

/// Most node definitions will work on numbers. This function makes sure that the input data is in a floating-point format
/// for consistent processing.
pub fn packet_to_f64(packet: &ViewValuePacket) -> Result<SmallVec<[f64; 6]>> {
    packet.iter()
        .map(|value| match value {
            ViewValue::F64(f) => Ok(*f),
            ViewValue::I64(i) => Ok(*i as f64),
            ViewValue::String(_) => Err(anyhow!("String input to block that expects numbers")),
        })
        .collect()
}

impl NodeDataset {
    /// Check if the number of values in the dataset is equal to the expected count.
    /// 
    /// This is useful, for example, to make sure that a node receives a specific number of
    /// inputs as required, before any processing.
    pub fn check_count(&self, count: usize) -> Result<&Self> {
        if self.packets.len() == count {
            Ok(self)
        } else {
            Err(anyhow!("Expected {} inputs, got {}", count, self.packets.len()))
        }
    }

    /// Call a function on each input packet of the dataset, producing a new output dataset
    /// 
    /// This is useful for generic nodes that will perform the same operation on an arbitrary number of inputs,
    /// producing the same number of outputs.
    pub fn map_values(&self, f: impl Fn(&ViewValuePacket) -> Result<ViewValuePacket>) -> Result<NodeDataset> {
        self.packets.iter()
            .map(f)
            .collect::<Result<_>>()
            .map(|packets| NodeDataset{ packets })
    }

    /// Create a new dataset with a single input packet.
    pub fn new_single(packet: ViewValuePacket) -> Self {
        NodeDataset {
            packets: SmallVec::from_vec(vec![packet])
        }
    }

    /// Create a new dataset based on a generator function.
    pub fn new_from_generator(n: usize, f: impl Fn() -> ViewValuePacket) -> Self {
        NodeDataset {
            packets: (0..n).map(|_| f()).collect()
        }
    }

    /// Create a new dataset based on a generator function that may return an error.
    pub fn try_new_from_generator(n: usize, f: impl Fn() -> Result<ViewValuePacket>) -> Result<Self> {
        Ok(NodeDataset {
            packets: (0..n).map(|_| f()).collect::<Result<_>>()?
        })
    }
}
