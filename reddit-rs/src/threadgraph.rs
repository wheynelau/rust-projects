use petgraph::Graph;
use std::collections::HashMap;
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::Dfs;

#[derive(Default)]
pub struct ThreadGraph {
    graph: Graph<String, ()>,
    node_map: HashMap<String, NodeIndex>,
    threads: Vec<NodeIndex>,
}

use crate::reddit::Reddit;
use crate::utils::writer::{JsonlWriter, JsonEntry};


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

    pub fn tranverse(&self, vec_threads: Vec<Reddit>, output: Option<&str>) {
        let mut threads_counter: usize = 0;
        
        let mut writer = output.map(|output| JsonlWriter::new(output).unwrap());

        for start in self.graph.node_indices() {
            let mut bfs = Dfs::new(&self.graph, start);

            let mut threads: Vec<usize> = Vec::new();
            
            while let Some(visited) = bfs.next(&self.graph) {
                threads.push(visited.index());
            }
    
            if threads.len() > 1 {
                let inner_long_string = threads
                .iter_mut()
                .map(|thread| vec_threads[*thread].selftext.clone())
                .collect::<Vec<_>>()
                .join("\n");
                
                if let Some(writer_ref) = writer.as_mut() {
                    let entry = JsonEntry {
                        length: inner_long_string.split_whitespace().count(),
                        raw_content: inner_long_string
                    };
                    writer_ref.write_line(entry).unwrap();
                }
                threads_counter += 1;
            }
        }
        if let Some(writer_ref) = writer.as_mut() {
            writer_ref.flush().unwrap();
        }
        // println!("Longest thread: {}", long_string);
        dbg!("Total threads: {}", threads_counter);
        // println!("Longest thread: {}", longest_thread);
        
    }
    
    pub fn show_threads(&self) {
        for node in self.graph.node_indices() {
            println!("{:?}", self.graph[node]);
        }
    }
    pub fn add_threads(&mut self, id: &String) {
        let idx = self.add_node(id);
        self.threads.push(idx);

    }
    pub fn is_in_map(&self, id: &String) -> bool {
        self.node_map.contains_key(id)
    }
    
}