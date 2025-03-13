use std::fmt::Debug;
use std::sync::Arc;
use smallvec::SmallVec;
use std::sync::RwLock;
use std::fmt;

use anyhow::{Context, Result};
use smallvec::smallvec;
use super::io::NodeDataset;

#[derive(Debug, Clone)]
pub enum NodeParameterValue {
    Number(f64),
    Integer(i64),
    String(String),
}

#[derive(Debug)]
pub enum NodeType {
    /// A processing node may have inputs and outputs
    Processing,
    /// An input node serves as the first input of other nodes, and itself has no inputs
    Input,
    /// An output node serves as the final output of other nodes, and itself has no outputs
    Output,
}

#[derive(Debug)]
pub struct NodeParameterDefinition {
    pub name: String,
    pub default_value: NodeParameterValue,
}

pub type NodeParameterSet = SmallVec<[NodeParameterValue; 6]>;

/// The main process function of a node. Takes the input dataset and produces the output dataset.
/// 
/// Arguments:
/// 1. Complete input dataset (with current values)
/// 2. Input parameter set (with current values)
/// 3. Recommended number of outputs. Useful e.g. for input blocks, so that the expected number of outputs is produced.
type NodeProcessFn = fn(&NodeDataset, &NodeParameterSet, usize) -> Result<NodeDataset>;

/// A definition of a node in the effect graph.
/// 
/// One definition can be instantiated multiple times in the graph.
#[derive(Debug)]
pub struct EffectNodeDefinition {
    pub name: String,
    pub help: Option<String>,
    pub parameters: Vec<NodeParameterDefinition>,
    pub node_type: NodeType,
    pub processor: NodeProcessFn,
}

/// Mark for visited nodes and their status during the execution of graph traversal algorithms
#[derive(Debug)]
pub(super) enum Mark {
    Unmarked,
    Temporary,
    Permanent,
}

/// An instantiation of a node in the effect graph.
/// 
/// It may be connected to other nodes as inputs or outputs. It is linked to one [EffectNodeDefinition].
pub struct EffectNode<'a> {
    pub label: String,
    pub definition: &'a EffectNodeDefinition,
    pub parameters: SmallVec<[NodeParameterValue; 6]>,
    /// Each input of the node has a different semantic definition and may be left unconnected.
    pub inputs: SmallVec<[Option<Arc<RwLock<EffectNode<'a>>>>; 3]>,
    /// Each output of the node may be connected to multiple other nodes.
    /// 
    /// Here, the outer array enumerates each semantically independent output, and the inner array
    /// contains all the connections of this output.
    pub outputs: SmallVec<[SmallVec<[Arc<RwLock<EffectNode<'a>>>; 3]>; 3]>,
    /// The x,y coordinates of the node in the node graph
    pub position: (f64, f64),
    /// The output value of the node, if calculated with [EffectNode::processor]
    pub current_value: NodeDataset,
    /// Marking variable used during traversal/search of the node graph
    pub(super) mark: Mark,
}

impl<'a> fmt::Debug for EffectNode<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Node \"{}\" [{}] = {:?}", self.label, self.definition.name, self.current_value)
    }
}

impl EffectNodeDefinition {
    pub fn build(&self, label: &str) -> EffectNode {
        EffectNode {
            label: label.to_string(),
            definition: self,
            parameters: self.parameters.iter().map(|param| param.default_value.clone()).collect(),
            inputs: SmallVec::new(),
            outputs: SmallVec::new(),
            position: (0.0, 0.0),
            current_value: NodeDataset{ packets: smallvec![] },
            mark: Mark::Unmarked,
        }
    }
}