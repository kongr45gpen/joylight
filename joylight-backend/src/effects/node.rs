use std::fmt::Debug;
use std::sync::Arc;
use smallvec::SmallVec;

use super::io::NodeDataPacket;
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

type NodeProcessFn = fn(&NodeDataset, &SmallVec<[NodeParameterValue; 6]>) -> Result<NodeDataset, ()>;

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

/// An instantiation of a node in the effect graph.
/// 
/// It may be connected to other nodes as inputs or outputs. It is linked to one [EffectNodeDefinition].
#[derive(Debug)]
pub struct EffectNode<'a> {
    pub label: String,
    pub definition: &'a EffectNodeDefinition,
    pub parameters: SmallVec<[NodeParameterValue; 6]>,
    /// Each input of the node has a different semantic definition and may be left unconnected.
    pub inputs: SmallVec<[Option<Arc<EffectNode<'a>>>; 3]>,
    /// Each output of the node may be connected to multiple other nodes.
    /// 
    /// Here, the outer array enumerates each semantically independent output, and the inner array
    /// contains all the connections of this output.
    pub outputs: SmallVec<[SmallVec<[Arc<EffectNode<'a>>; 3]>; 3]>,
    pub position: (f64, f64),
    pub current_value: NodeDataset,
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
            current_value: NodeDataset(SmallVec::new()),
        }
    }
}