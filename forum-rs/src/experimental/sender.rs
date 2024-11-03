use crossbeam_channel::{unbounded, Receiver, Sender};
use rayon::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::forum_thread;
use crate::graph::ThreadGraph;
use crate::utils;

#[allow(dead_code)]
fn process_line(rx: Receiver<String>, tx: Sender<forum_thread::Post>) {
    while let Ok(line) = rx.recv() {
        if let Ok(json) = serde_json::from_str::<forum_thread::JsonStruct>(&line) {
            if let Some(thread) = forum_thread::Post::from_json_struct(json) {
                tx.send(thread).unwrap();
            }
        }
    }
}

fn process_graph(
    rx: Receiver<forum_thread::Post>,
    mut threadgraph: ThreadGraph,
    mut comments: Vec<forum_thread::Post>,
) -> (ThreadGraph, Vec<forum_thread::Post>) {
    while let Ok(thread) = rx.recv() {
        let thread_node = threadgraph.add_node(thread.clone());
        match thread.is_thread {
            true => threadgraph.add_threads(thread_node),
            false => comments.push(thread),
        }
    }
    (threadgraph, comments)
}

pub fn get_threads(path: &str) -> Vec<(String, Vec<String>)> {
    let entries = utils::file::single_folder(path);
    let (post_tx, post_rx) = unbounded();
    // let (string_tx, string_rx) = bounded(1000);
    let threadgraph = ThreadGraph::new();
    let comments: Vec<forum_thread::Post> = Vec::with_capacity(10000);

    // let line_handle = std::thread::spawn(move || {
    //     process_line(string_rx, post_tx);
    // });

    let graph_handle = std::thread::spawn(move || process_graph(post_rx, threadgraph, comments));
    // let threadgraph = Arc::new(Mutex::new(graph::ThreadGraph::new()));
    // let comments = Arc::new(Mutex::new(Vec::with_capacity(10000)));
    // this shouldn't be parallelized for safety
    entries.par_iter().for_each(|entry| {
        let fp = File::open(entry).unwrap();
        let reader = BufReader::new(fp);

        reader
            .lines()
            .par_bridge()
            .filter_map(|line| line.ok())
            .filter_map(|line| {
                serde_json::from_str::<forum_thread::JsonStruct>(&line)
                    .ok()
                    .and_then(forum_thread::Post::from_json_struct)
            })
            .for_each(|post| {
                post_tx.send(post).unwrap();
            });
    });

    // Drop the sender to signal the end of the stream
    drop(post_tx);

    // Wait for the graph processing to complete
    let (mut threadgraph, comments) = graph_handle.join().unwrap();

    for comment in comments.iter() {
        threadgraph.add_edge(&comment.parent_post_id, &comment.id);
    }

    threadgraph.traverse()
}
