use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::Dfs;
use petgraph::Graph;
use std::collections::{HashMap, HashSet};


use crate::thread::Post;

#[derive(Default)]
pub struct ThreadGraph {
    graph: Graph<String, ()>,
    node_map: HashMap<String, NodeIndex>,
    threads: Vec<NodeIndex>,
    allthreads: Vec<Post>,
}

impl ThreadGraph {
    pub fn new() -> Self {
        ThreadGraph {
            graph: DiGraph::new(),
            node_map: HashMap::new(),
            threads: Vec::new(),
            allthreads: Vec::new(),
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

    fn check_duplicates(&self) -> bool {
        let mut set: HashSet<&String> = HashSet::new();
        self.graph.node_indices().all(|node| set.insert(&self.graph[node]));
        set.len() == self.graph.node_count()
    }

    fn show_roots(&self) {
        let mut roots = 0;
        for node in self.graph.node_indices() {
            if self.graph.neighbors_directed(node, petgraph::Direction::Incoming).count() == 0 {
                println!("{:?}", self.graph[node]);
                roots += 1;
            }
        }
        dbg!(roots);
        dbg!(self.threads.len());
        
    }

    pub fn tranverse(&self, vec_threads: Vec<Post>) -> Vec<(String, Vec<String>)> {

        // check for duplicates
        self.show_roots();

        // print number of nodes
        dbg!(self.graph.node_count());
        let mut final_threads: Vec<(String, Vec<String>)> = Vec::with_capacity(10000);
        for start in self.threads.iter() {
            let mut dfs = Dfs::new(&self.graph, *start);

            let mut threads: Vec<usize> = Vec::new();

            while let Some(visited) = dfs.next(&self.graph) {
                threads.push(visited.index());
            }

            if threads.len() > 1 {
                // get first idx for the root
                // let root_thread = &vec_threads[threads[0]];
                let vec_string: Vec<String> = threads
                    .iter()
                    .map(|thread| 
                        {
                            // print!("{} ", thread);
                            vec_threads[*thread].pagetext.clone()
                        })
                    .collect();
                final_threads.push(("test".to_string(), vec_string));
                // println!();
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
