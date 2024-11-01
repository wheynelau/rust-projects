use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::Dfs;
use petgraph::Graph;
use std::collections::HashMap;

use crate::thread::Post;

#[derive(Default)]
pub struct ThreadGraph {
    graph: Graph<String, ()>,
    node_map: HashMap<String, NodeIndex>,
    threads: Vec<NodeIndex>,
}

impl ThreadGraph {
    pub fn new() -> Self {
        ThreadGraph {
            graph: DiGraph::new(),
            node_map: HashMap::new(),
            threads: Vec::new(),
        }
    }

    pub fn add_node(&mut self, id: &String) -> NodeIndex {
        if let Some(&idx) = self.node_map.get(id) {
            idx
        } else {
            let idx = self.graph.add_node(id.clone());
            self.node_map.insert(id.to_string(), idx);
            idx
        }
    }

    pub fn add_edge(&mut self, from_id: &String, to_id: &String) {
        let from_idx = self.add_node(from_id);
        let to_idx = self.add_node(to_id);
        self.graph.add_edge(from_idx, to_idx, ());
    }

    pub fn tranverse(&self, vec_threads: Vec<Post>) -> Vec<(String, Vec<String>)> {
        let mut threads_counter: usize = 0;
        let mut final_threads: Vec<(String, Vec<String>)> = Vec::with_capacity(10000);
        for start in self.threads.iter() {
            let mut bfs = Dfs::new(&self.graph, *start);

            let mut threads: Vec<usize> = Vec::new();

            while let Some(visited) = bfs.next(&self.graph) {
                threads.push(visited.index());
            }

            if threads.len() > 1 {
                // get first idx for the root
                let root_thread = &vec_threads[threads[0]];
                let vec_string: Vec<String> = threads
                    .into_iter()
                    .map(|thread| vec_threads[thread].pagetext.clone())
                    .collect();
                threads_counter += 1;
                final_threads.push((root_thread.id.clone(), vec_string));
            }
        }
        // println!("Longest thread: {}", long_string);
        // println!("Longest thread: {}", longest_thread);
        final_threads
    }

    pub fn show_threads(&self) {
        for node in self.graph.node_indices() {
            println!("{:?}", self.graph[node]);
        }
    }
    pub fn add_threads(&mut self, idx: NodeIndex) {
        self.threads.push(idx);
    }
    pub fn is_in_map(&self, id: &String) -> bool {
        self.node_map.contains_key(id)
    }
}
