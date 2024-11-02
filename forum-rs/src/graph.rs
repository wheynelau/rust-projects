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
    id_set: HashSet<String>,
}

impl ThreadGraph {
    pub fn new() -> Self {
        ThreadGraph {
            graph: DiGraph::new(),
            node_map: HashMap::new(),
            threads: Vec::new(),
            allthreads: Vec::new(),
            id_set: HashSet::new(),
        }
    }

    pub fn add_node(&mut self, post: Post) -> NodeIndex {
        let id = post.id.clone();
        if let Some(&idx) = self.node_map.get(&id) {
            idx
        } else {
            let idx = self.graph.add_node(id.clone());
            self.allthreads.push(post);
            self.id_set.insert(id.clone());
            self.node_map.insert(id.to_string(), idx);
            idx
        }
    }

    pub fn add_edge(&mut self, from_id: &String, to_id: &String) {
        // check if from_id is in map
        if !self.id_set.contains(from_id) {
            // create a placeholder Post
            let post = Post::placeholder(from_id.to_string());
            let idx = self.add_node(post);
            self.add_threads(idx)
        }
        let from_idx = self
            .node_map
            .get(from_id)
            .expect("from_id should exist at this point");
        let to_idx = self
            .node_map
            .get(to_id)
            .expect("to_id should exist at this point");

        // Add the edge
        self.graph.add_edge(*from_idx, *to_idx, ());
    }

    #[allow(dead_code)]
    fn check_duplicates(&self) -> bool {
        let mut set: HashSet<&String> = HashSet::new();
        self.graph
            .node_indices()
            .all(|node| set.insert(&self.graph[node]));
        set.len() == self.graph.node_count()
    }

    #[allow(dead_code)]
    pub fn show_roots(&self) -> Vec<NodeIndex> {
        let mut roots_idx: Vec<NodeIndex> = Vec::new();
        for node in self.graph.node_indices() {
            let incoming_count = self
                .graph
                .neighbors_directed(node, petgraph::Direction::Incoming)
                .count();

            if incoming_count == 0 {
                roots_idx.push(node);
            }
        }
        println!("Found {} roots: {:?}", roots_idx.len(), &roots_idx);
        roots_idx
    }

    pub fn tranverse(&self) -> Vec<(String, Vec<String>)> {
        // check for duplicates
        // self.show_roots();
        // let mut root_id: String = String::new();
        // print number of nodes
        // dbg!(self.graph.node_count());
        let mut final_threads: Vec<(String, Vec<String>)> = Vec::with_capacity(10000);
        for start in self.threads.iter() {
            // skip if not root
            let mut dfs = Dfs::new(&self.graph, *start);
            let mut threads: Vec<usize> = Vec::new();

            while let Some(visited) = dfs.next(&self.graph) {
                threads.push(visited.index());
            }
            let root_id = self.graph[*start].clone();
            let vec_string: Vec<String> = threads
                .iter()
                .map(|thread| {
                    // print!("{} ", thread);
                    self.allthreads[*thread].pagetext.clone()
                })
                .collect();
            // dbg!(vec_string.len());
            // println!();
            final_threads.push((root_id, vec_string));
        }
        // println!("Longest thread: {}", long_string);
        // println!("Longest thread: {}", longest_thread);
        // dbg!(roots);
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
