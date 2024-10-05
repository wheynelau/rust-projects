use petgraph::Graph;
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use petgraph::graph::{DiGraph, NodeIndex, UnGraph};
use petgraph::visit::{Dfs, DfsPostOrder, IntoNeighborsDirected};

pub struct ThreadGraph {
    graph: Graph<Arc<String>, ()>,
    node_map: HashMap<Arc<String>, NodeIndex>,
    threads: Vec<NodeIndex>,
}

impl ThreadGraph {
    pub fn new() -> Self {
        ThreadGraph {
            graph: Graph::new(),
            node_map: HashMap::new(),
            threads: Vec::new(),
        }
    }

    pub fn add_node(&mut self, id: String) -> NodeIndex {
        let id = Arc::new(id);
        if let Some(&idx) = self.node_map.get(&id) {
            idx
        } else {
            let idx = self.graph.add_node(Arc::clone(&id));
            self.node_map.insert(id, idx);
            idx
        }
    }

    pub fn add_edge(&mut self, from_id: String, to_id: String) {
        let from_idx = self.add_node(from_id);
        let to_idx = self.add_node(to_id);
        self.graph.add_edge(from_idx, to_idx, ());
    }

    pub fn tranverse(&self) { // Assuming 0 is the root node

        for start in self.threads.iter() {
            let mut dfs = Dfs::new(&self.graph, *start);
    
            print!("[{}] ", start.index());
    
            while let Some(visited) = dfs.next(&self.graph) {
                print!(" {}", visited.index());
            }
    
            println!();
        }

        // // Print leaf paths
        // for path in leaf_paths {
        //     print!("[{}] ", path.back().unwrap().index());
        //     for node in path {
        //         print!(" {}", node.index());
        //     }
        //     println!(" <-");
        // }
    }
    pub fn show_threads(&self) {
        for node in self.graph.node_indices() {
            println!("{:?}", self.graph[node]);
        }
    }
    pub fn add_threads(&mut self, id: String){
        let idx = self.add_node(id);
        self.threads.push(idx);

    }
    pub fn is_in_map(&self, id: &str) -> bool {
        self.node_map.contains_key(&Arc::new(id.to_string()))
    }   
}