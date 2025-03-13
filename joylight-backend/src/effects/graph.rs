use std::collections::LinkedList;
use std::fmt::Debug;
use std::sync::Arc;
use std::sync::RwLock;

use anyhow::Context;
use smallvec::smallvec;
use smallvec::SmallVec;
use anyhow::{anyhow,Result};
use log::{debug, info};

use crate::effects::io::NodeDataset;
use crate::effects::node::EffectNode;
use crate::effects::node::Mark;
use crate::effects::node::NodeParameterValue;
use crate::parameter::parameter_view::ViewValue;

use std::io::Write;
use std::process::{Command, Stdio};

#[derive(Debug)]
pub struct EffectGraph<'a> {
    sorted: bool,
    pub nodes: Vec<Arc<RwLock<EffectNode<'a>>>>,
}

impl<'a> EffectGraph<'a> {
    pub fn new() -> Self {
        EffectGraph {
            sorted: false,
            nodes: vec![],
        }
    }

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
        println!("{}", String::from_utf8_lossy(&output.stdout));

        graphviz
    }

    pub fn add_node(&mut self, node: EffectNode<'a>) -> Arc<RwLock<EffectNode<'a>>> {
        let new_node = Arc::new(RwLock::new(node));

        self.nodes.push(new_node.clone());
        self.sorted = false;

        new_node
    }

    fn node_topological_sort(&mut self) -> Result<()> {
        if self.nodes.is_empty() {
            return Err(anyhow!("Effect graph is empty"));
        }

        let mut nodes_sorted = vec![];
        nodes_sorted.reserve(self.nodes.len());

        struct Visit<'a>(Arc<RwLock<EffectNode<'a>>>, bool);

        let mut node_stack: LinkedList<Visit<'a>> = LinkedList::new();
        node_stack.push_back(Visit(self.nodes.first().unwrap().clone(), false));

        // Iterative depth-first search to sort the graph topologically
        // Cormen et al. (2001), see https://en.wikipedia.org/wiki/Topological_sorting#Depth-first_search
        while !node_stack.is_empty() {
            {
                let current_node_container = node_stack.pop_back().unwrap();
                let mut current_node = current_node_container.0.write().unwrap();

                if current_node_container.1 {
                    current_node.mark = Mark::Permanent;
                    nodes_sorted.push(current_node_container.0.clone());
                } else {
                    match current_node.mark {
                        Mark::Permanent => {
                            continue;
                        }
                        Mark::Temporary => {
                            return Err(anyhow!("Effect graph contains a cycle"));
                        }
                        Mark::Unmarked => {
                            current_node.mark = Mark::Temporary;

                            node_stack.push_back(Visit(current_node_container.0.clone(), true));

                            for input in current_node.inputs.iter() {
                                if let Some(input) = input {
                                    node_stack.push_back(Visit(input.node.clone(), false));

                                    if input.output_id >= input.node.read().unwrap().output_count {
                                        input.node.write().unwrap().output_count = input.output_id + 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // If the graph is disconnected
            if node_stack.is_empty() && nodes_sorted.len() < self.nodes.len() {
                for node_container in self.nodes.iter() {
                    let node = node_container.read().unwrap();

                    match &node.mark {
                        Mark::Unmarked => {
                            node_stack.push_back(Visit(node_container.clone(), false));
                            break;
                        }
                        _ => {}
                    }
                }
            }
        }

        self.nodes = nodes_sorted;
        self.sorted = true;

        debug!("Evaluation order of FX graph: {}", self.nodes.iter().map(|node| node.read().unwrap().label.clone()).collect::<Vec<_>>().join(", "));

        Ok(())
    }

    pub fn process(&mut self) {
        if !self.sorted {
            self.node_topological_sort().unwrap();
        }

        for node in self.nodes.iter() {
            let mut node = node.write().unwrap();
            debug!("Processing node: {:?}", node);
            let null_parameters: SmallVec<[NodeParameterValue; 6]> = SmallVec::new();

            let input = node
                .inputs
                .iter()
                .map(|input| {
                    debug!("  Input: {:?}", input);
                    match input {
                        Some(input) => {
                            let input = input.node.read().unwrap();
                            Some(input.current_value.packets[0].clone().unwrap())
                        }
                        _ => Some(smallvec![ViewValue::F64(0.0)]),
                    }
                })
                .collect();

            let input_dataset = NodeDataset{ packets: input };

            let output = (node.definition.processor)(&input_dataset, &null_parameters, node.output_count)
                .with_context(|| format!("Error processing node: {}", node.label));
            debug!(" Output: {:?}", output);

            node.current_value = output.unwrap();
        }
    }
}
