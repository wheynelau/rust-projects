use crate::forum_thread::Post;
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::Dfs;
use petgraph::Graph;
use rayon::prelude::*;
use std::collections::HashMap;

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
            node_map: HashMap::with_capacity(10000),
            threads: Vec::with_capacity(10000),
            allthreads: Vec::with_capacity(10000),
        }
    }

    pub fn add_node(&mut self, post: Post) -> NodeIndex {
        let id = post.id.clone();
        if let Some(&idx) = self.node_map.get(&id) {
            idx
        } else {
            let idx = self.graph.add_node(id.clone());
            self.allthreads.push(post);
            // self.id_set.insert(id.clone());
            self.node_map.insert(id.to_string(), idx);
            idx
        }
    }

    pub fn add_edge(&mut self, from_id: &String, to_id: &String) {
        // check if from_id is in map
        if !self.node_map.contains_key(from_id) {
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

    // #[allow(dead_code)]
    // fn check_duplicates(&self) -> bool {
    //     let mut set: HashSet<&String> = HashSet::new();
    //     self.graph
    //         .node_indices()
    //         .all(|node| set.insert(&self.graph[node]));
    //     set.len() == self.graph.node_count()
    // }

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

    pub fn traverse(&self) -> Vec<(String, Vec<String>)> {
        // check for duplicates
        // self.show_roots();
        // let mut root_id: String = String::new();
        // print number of nodes
        //dbg!(self.graph.node_count());

        let mut final_threads: Vec<(String, Vec<String>)> = Vec::with_capacity(self.threads.len());
        self.threads
            .par_iter()
            .with_min_len(100)
            .map(|start| {
                // skip if not root
                let mut dfs = Dfs::new(&self.graph, *start);
                let mut threads: Vec<usize> = Vec::new();

                while let Some(visited) = dfs.next(&self.graph) {
                    threads.push(visited.index());
                }
                let root_id = self.graph[*start].clone();
                let vec_string: Vec<String> = threads
                    .iter()
                    // .with_min_len(100)
                    .map(|thread| {
                        // print!("{} ", thread);
                        self.allthreads[*thread].pagetext.clone()
                    })
                    .collect();
                // dbg!(vec_string.len());
                // println!();
                (root_id, vec_string)
            })
            .collect_into_vec(&mut final_threads);
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
#[cfg(test)]
mod tests {
    use super::*;
    use itertools::izip;
    use rand::prelude::*;

    fn setup() -> (ThreadGraph, Vec<Post>) {
        let test_cases = vec![
            // first graph, basic 1>2&3
            ("1", true, "1", "1", "1"),
            ("2", true, "2", "2", "2"),
            ("3", false, "3", "1", "1"),
            ("4", false, "4", "3", "3"),
            ("5", false, "5", "3", "3"),
            ("6", false, "6", "4", "4"),
            ("7", false, "7", "2", "2"),
            ("8", false, "8", "7", "7"),
            ("9", false, "9", "7", "7"),
            ("10", false, "10", "8", "8"),
            // detached thread
            ("11", false, "11", "12", "12"),
        ];

        let graph = ThreadGraph::new();
        let posts = test_cases
            .into_iter()
            .map(|(id, is_thread, pagetext, parent_post_id, root_post_id)| {
                Post::new(id, is_thread, pagetext, parent_post_id, root_post_id)
            })
            .collect();
        (graph, posts)
    }
    #[test]
    fn test_functional_graph() {
        // TODO: There should be a more idiomatic way to do this
        // assumes dfs
        let mut target: Vec<(&str, Vec<&str>)> = vec![
            ("2", vec!["2", "7", "9", "8", "10"]),
            ("1", vec!["1", "3", "5", "4", "6"]),
            ("12", vec!["", "11"]),
        ];
        let mut alternative_target: Vec<(&str, Vec<&str>)> = vec![
            ("2", vec!["2", "7", "8", "10", "9"]),
            ("1", vec!["1", "3", "4", "6", "5"]),
            ("12", vec!["11", ""]),
        ];
        // sort target
        target.sort_by(|a, b| a.0.cmp(&b.0));
        alternative_target.sort_by(|a, b| a.0.cmp(&b.0));

        // run a loop for better determinism
        for _ in 0..10 {
            let (mut graph, mut posts) = setup();
            posts.shuffle(&mut thread_rng());
            let mut comments = Vec::new();
            for post in posts.iter() {
                let idx = graph.add_node(post.clone());
                match post.is_thread {
                    true => graph.add_threads(idx),
                    false => comments.push(post.clone()),
                }
            }
            assert_eq!(graph.graph.node_count(), 11);
            assert_eq!(graph.threads.len(), 2);
            assert_eq!(comments.len(), 9);

            // add edges
            for comment in comments.iter() {
                graph.add_edge(&comment.parent_post_id, &comment.id);
            }
            // should be 12 due to detached thread
            assert_eq!(graph.graph.node_count(), 12);
            assert_eq!(graph.graph.edge_count(), 9);
            assert_eq!(graph.threads.len(), 3);

            let mut threads = graph.traverse();
            threads.sort_by(|a, b| a.0.cmp(&b.0));

            assert_eq!(threads.len(), target.len());

            // check against target and alternative target
            for (result, x, y) in izip!(threads, &target, &alternative_target) {
                assert_eq!(result.0, x.0);
                assert!(result.1 == x.1 || result.1 == y.1);
            }
        }
    }
}
