use smallvec::SmallVec;
use std::fmt::Debug;

use crate::parameter::parameter_view::ViewValue;

/// A node data packet is fed at regular intervals as input and can be produced as output by nodes.
#[derive(Debug, Clone)]
pub struct NodeDataPacket(pub SmallVec<[ViewValue; 6]>);

/// A node may have multiple inputs and outputs that may themselves be vectors.
/// A "dataset" represents all the set of [NodeDataPacket]s that are input or output by the node.
#[derive(Debug, Clone)]
pub struct NodeDataset(pub SmallVec<[Option<NodeDataPacket>; 3]>);

impl NodeDataPacket {
    /// Most node definitions will work on numbers. This function makes sure that the input data is in a floating-point format
    /// for consistent processing.
    ///
    /// TODO: There is a more efficient way to do this without having to push to an array
    pub fn process_as_floats(&self) -> Result<SmallVec<[f64; 6]>, ()> {
        let mut new_values = SmallVec::new();

        for value in &(self.0) {
            match value {
                ViewValue::F64(f) => new_values.push(*f),
                ViewValue::I64(i) => new_values.push(*i as f64),
                ViewValue::String(_) => return Err(()),
            }
        }

        Ok(new_values)
    }
}

impl NodeDataset {
    pub fn check_count(&self, count: usize) -> Result<(), ()> {
        if self.0.len() == count {
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn map(&self, f: impl Fn(&NodeDataPacket) -> Result<NodeDataPacket, ()>) -> Result<NodeDataset, ()> {
        let mut result = SmallVec::with_capacity(self.0.len());
        
        for input in &self.0 {
            match input {
                Some(input) => match f(input) {
                    Ok(new_input) => result.push(Some(new_input)),
                    Err(_) => return Err(()),
                },
                None => result.push(None),
            }
        }

        Ok(NodeDataset(result))
    }
}
