use std::collections::LinkedList;
use std::fmt::Debug;
use std::sync::Arc;
use std::sync::RwLock;

use smallvec::smallvec;
use smallvec::SmallVec;
use zmq::Error;

use crate::effects::io::NodeDataset;
use crate::effects::node::EffectNode;
use crate::effects::node::Mark;
use crate::effects::node::NodeParameterValue;
use crate::parameter::parameter_view::ViewValue;

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

    pub fn add_node(&mut self, node: EffectNode<'a>) -> Arc<RwLock<EffectNode<'a>>> {
        let new_node = Arc::new(RwLock::new(node));

        self.nodes.push(new_node.clone());
        self.sorted = false;

        new_node
    }

    fn node_topological_sort(&mut self) -> Result<(), ()> {
        if self.nodes.is_empty() {
            return Err(());
        }

        let mut nodes_sorted = vec![];
        nodes_sorted.reserve(self.nodes.len());

        struct Visit<'a>(Arc<RwLock<EffectNode<'a>>>, bool);

        let mut node_stack: LinkedList<Visit<'a>> = LinkedList::new();
        node_stack.push_back(Visit(self.nodes.first().unwrap().clone(), false));

        // Iterative depth-first search to sort the graph topologically
        // Cormen et al. (2001)
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
                            return Err(());
                        }
                        Mark::Unmarked => {
                            current_node.mark = Mark::Temporary;

                            node_stack.push_back(Visit(current_node_container.0.clone(), true));

                            for output in current_node.outputs.iter() {
                                for connection in output.iter() {
                                    node_stack.push_back(Visit(connection.clone(), false));
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

        Ok(())
    }

    pub fn process(&mut self) {
        println!("Nodes before topological sort: {:#?}", self.nodes);
        if !self.sorted {
            self.node_topological_sort().unwrap();
        }
        println!("Nodes after topological sort: {:#?}", self.nodes);

        for node in self.nodes.iter().rev() {
            let mut node = node.write().unwrap();
            println!("Processing node: {:?}", node);
            let null_parameters: SmallVec<[NodeParameterValue; 6]> = SmallVec::new();

            let input = node
                .inputs
                .iter()
                .map(|input| {
                    println!("  Input: {:?}", input);
                    match input {
                        Some(input) => {
                            let input = input.read().unwrap();
                            Some(input.current_value.packets[0].clone().unwrap())
                        }
                        _ => Some(smallvec![ViewValue::F64(0.0)]),
                    }
                })
                .collect();

            let input_dataset = NodeDataset{ packets: input };

            let output = (node.definition.processor)(&input_dataset, &null_parameters, node.outputs.len());
            println!(" Output: {:?}", output);
            node.current_value = output.unwrap();
        }
    }
}
