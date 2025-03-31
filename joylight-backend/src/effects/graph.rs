use std::collections::LinkedList;
use std::fmt::Debug;
use std::io::Write;
use std::process::{Command, Stdio};
use std::sync::{Arc, RwLock};

use anyhow::{anyhow, Context, Result};
use log::{debug, info};
use smallvec::{smallvec, SmallVec};

use crate::effects::io::NodeDataset;
use crate::effects::node::{EffectNode, Mark, NodeParameterValue, NodeRef};

#[derive(Debug)]
pub struct EffectGraph<'a> {
    sorted: bool,
    pub nodes: Vec<NodeRef<'a>>,
}

impl Default for EffectGraph<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> EffectGraph<'a> {
    pub fn new() -> Self {
        EffectGraph {
            sorted: false,
            nodes: vec![],
        }
    }

    /// Run "graph-easy" in Perl to print an ASCII representation of the graph in the terminal
    pub fn graphviz(&self) -> String {
        let mut graphviz = String::new();
        graphviz.push_str("digraph G {\n");

        for node in self.nodes.iter() {
            let node = node.read().unwrap();
            graphviz.push_str(&format!("  {} [label=\"{}\"];\n", node.label, node.label));

            for input in node.inputs.iter() {
                for connection in input.iter() {
                    let connection = connection.node.read().unwrap();
                    graphviz.push_str(&format!("  {} -> {};\n", node.label, connection.label));
                }
            }
        }

        graphviz.push_str("}\n");

        let mut child = Command::new("graph-easy")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to execute dot");

        let stdin = child.stdin.as_mut().unwrap();
        stdin.write_all(graphviz.as_bytes()).unwrap();

        let output = child.wait_with_output().unwrap();
        String::from_utf8_lossy(&output.stdout).into()
    }

    /// Add a node to the effects graph
    ///
    /// This will break the topological sort of the graph.
    pub fn add_node(&mut self, node: EffectNode<'a>) -> NodeRef<'a> {
        let new_node = Arc::new(RwLock::new(node));

        self.nodes.push(new_node.clone());
        self.sorted = false;

        new_node
    }

    /// Topologically sort all nodes in the graph
    ///
    /// This function replaces `self.nodes` with a version sorted so that nodes always come after their
    /// dependencies. You should call this
    ///
    /// It will return an error when the sort is not possible, for example, when there is a cycle.
    fn node_topological_sort(&mut self) -> Result<()> {
        if self.nodes.is_empty() {
            return Err(anyhow!("Effect graph is empty"));
        }

        let mut nodes_sorted = Vec::with_capacity(self.nodes.len());

        #[derive(Clone)]
        struct Visit<'a>(NodeRef<'a>, bool);

        let mut node_stack: LinkedList<Visit<'a>> = LinkedList::new();
        node_stack.extend(self.nodes.iter().map(|node| Visit(node.clone(), false)));

        // Reset node properties
        for node in self.nodes.iter() {
            let mut node: std::sync::RwLockWriteGuard<'_, EffectNode<'a>> = node.write().unwrap();
            node.mark = Mark::Unmarked;
            node.output_count = 0;
        }

        fn visit_node<'a>(
            visit: &Visit<'a>,
            nodes_sorted: &mut Vec<NodeRef<'a>>,
            node_stack: &mut LinkedList<Visit<'a>>,
        ) -> Result<()> {
            let mut current_node = visit.0.write().unwrap();

            if visit.1 {
                current_node.mark = Mark::Permanent;
                nodes_sorted.push(visit.0.clone());

                Ok(())
            } else {
                match current_node.mark {
                    Mark::Permanent => {
                        return Ok(());
                    }
                    Mark::Temporary => {
                        return Err(anyhow!("Effect graph contains a cycle"));
                    }
                    Mark::Unmarked => {
                        current_node.mark = Mark::Temporary;

                        node_stack.push_back(Visit(visit.0.clone(), true));

                        for input in current_node.inputs.iter().flatten() {
                            node_stack.push_back(Visit(input.node.clone(), false));

                            if input.output_id >= input.node.read().unwrap().output_count {
                                let mut node = input.node.write().unwrap();
                                node.output_count = input.output_id + 1;
                            }
                        }
                    }
                }

                Ok(())
            }
        }

        // Iterative depth-first search to sort the graph topologically
        // Cormen et al. (2001), see https://en.wikipedia.org/wiki/Topological_sorting#Depth-first_search
        node_stack.push_back(Visit(self.nodes.first().unwrap().clone(), false));

        while !node_stack.is_empty() {
            let visit = node_stack.pop_back().unwrap();
            visit_node(&visit, &mut nodes_sorted, &mut node_stack)?;
        }

        self.nodes = nodes_sorted;
        self.sorted = true;

        debug!(
            "Evaluation order of FX graph: {}",
            self.nodes
                .iter()
                .map(|node| node.read().unwrap().label.clone())
                .collect::<Vec<_>>()
                .join(", ")
        );

        Ok(())
    }

    /// Process all nodes in the graph, storing their new state in the node instance
    pub fn process(&mut self) -> Result<()> {
        if !self.sorted {
            self.node_topological_sort().unwrap();
        }

        let null_parameters: SmallVec<[NodeParameterValue; 6]> = SmallVec::new();

        for node in self.nodes.iter() {
            let mut node = node.write().unwrap();
            debug!("Processing node: {}", node.label);

            let input_data = node
                .inputs
                .iter()
                .map(|input| {
                    // for each option<input>
                    match input {
                        Some(link) => {
                            let from_node = link.node.read().unwrap();
                            from_node
                                .current_value
                                .packets
                                .get(link.output_id)
                                .ok_or(anyhow!(
                                    "Asked for output {} from node `{}`, but it only has {} outputs",
                                    link.output_id,
                                    from_node.label,
                                    from_node.current_value.packets.len()
                                ))
                                .cloned()
                        }
                        None => Ok(smallvec![]),
                    }
                })
                .collect::<Result<SmallVec<_>>>()
                .with_context(|| format!("Error processing node: {}", node.label))?;

            let input_dataset = NodeDataset { packets: input_data };

            let output = (node.definition.processor)(&input_dataset, &null_parameters, node.output_count)
                .with_context(|| format!("Error processing node: {}", node.label));
            debug!(" Output: {:?}", output);

            node.current_value = output?;
        }

        Ok(())
    }
}
