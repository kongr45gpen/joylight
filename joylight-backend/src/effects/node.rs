use smallvec::SmallVec;
use std::fmt;
use std::fmt::Debug;
use std::sync::Arc;
use std::sync::RwLock;

use super::io::NodeDataset;
use anyhow::{Context, Result};
use smallvec::smallvec;

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

/// An Arc reference to an effects node
type NodeRef<'a> = Arc<RwLock<EffectNode<'a>>>;

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

/// Denotes the input of a node, which is another node
#[derive(Debug)]
pub struct InputLink<'a> {
    pub node: NodeRef<'a>,
    /// Defines which output of the inputting node is connected to this input
    pub output_id: usize,
}

/// An instantiation of a node in the effect graph.
///
/// It may be connected to other nodes as inputs or outputs. It is linked to one [EffectNodeDefinition].
pub struct EffectNode<'a> {
    pub label: String,
    pub definition: &'a EffectNodeDefinition,
    pub parameters: NodeParameterSet,
    /// Each input of the node has a different semantic definition and may be left unconnected.
    pub inputs: SmallVec<[Option<InputLink<'a>>; 3]>,
    /// The maximum number of distinct outputs provided by this node
    pub output_count: usize,
    /// The x,y coordinates of the node in the node graph
    pub position: (f64, f64),
    /// The output value of the node, if calculated with [EffectNode::processor]
    pub current_value: NodeDataset,
    /// Marking variable used during traversal/search of the node graph
    pub(super) mark: Mark,
}

impl<'a> fmt::Debug for EffectNode<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Node \"{}\" [{}] = {:?}",
            self.label, self.definition.name, self.current_value
        )
    }
}

impl EffectNodeDefinition {
    /// Create a new [EffectNode] instance from a definition
    pub fn build(&self, label: &str) -> EffectNode {
        EffectNode {
            label: label.to_string(),
            definition: self,
            parameters: self
                .parameters
                .iter()
                .map(|param| param.default_value.clone())
                .collect(),
            inputs: SmallVec::new(),
            output_count: 0,
            position: (0.0, 0.0),
            current_value: NodeDataset {
                packets: smallvec![],
            },
            mark: Mark::Unmarked,
        }
    }
}

/// Connect the `n`th output of `from` to a new input of `to`
pub fn link<'a>(from: &NodeRef<'a>, to: &NodeRef<'a>, n: usize) {
    to.write().unwrap().inputs.push(Some(InputLink {
        node: from.clone(),
        output_id: n,
    }));
}
