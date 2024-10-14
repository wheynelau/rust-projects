use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Mul;
use std::sync::{Arc, Mutex};
use zstd::Decoder;
use serde_json::Value;
use petgraph::graph::{NodeIndex};
use rayon::prelude::*;


mod threadgraph;
use threadgraph::{ThreadGraph, Reddit, Thread, Comment};

fn main() -> std::io::Result<()> {
    let thread_file = File::open("data/askSingapore_submissions.zst")?;
    let comment_file = File::open("data/askSingapore_comments.zst")?;
    let reader = create_file_buffer(thread_file);
    let mut threads: Vec<Thread> = Vec::new();
    let comments = Vec::new();
    let mut thread_graph = threadgraph::ThreadGraph::new();
    // just print the first 10 lines
    let _ = thread_graph.add_node("root");
    for (index, line) in reader.lines().enumerate() {
        if index == 10 {
            break;
        }
        let line = line.unwrap();
        let json: Value = serde_json::from_str(&line).unwrap();
        let thread = create_thread(json);
        if let Some(thread) = thread {
            thread_graph.add_threads(&thread.id);
            thread_graph.add_edge("root", &thread.id);
            threads.push(thread);
        }

    }
    let reader = create_file_buffer(comment_file);
    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();
    dbg!(lines.len());
    let lines: Vec<Comment> = lines.par_iter()
    .filter_map(|line| {
        let json: Result<Value, _> = serde_json::from_str(line);
        match json {
            Ok(json) => handle_comment(json),
            Err(_) => None,
        }
    })
    .collect();

    for comment in lines {
            if thread_graph.is_in_map(&comment.parent_id) {
                thread_graph.add_node(&comment.id);
                thread_graph.add_edge(&comment.parent_id,&comment.id);
            }

    };

    thread_graph.tranverse(threads,comments);
    Ok(())
}

fn create_file_buffer(file: File) -> BufReader<zstd::Decoder<'static, BufReader<File>>> {
    let decoder = Decoder::new(file).unwrap();
    let reader = BufReader::new(decoder);
    reader
}

fn create_thread(json: Value) -> Option<Thread> {
    if let Some(num_comments) = json.get("num_comments") {
        let num_comments = num_comments.as_i64().unwrap();
        
        if num_comments == 0 {
            return None;
        }
        
        let id = json.get("id").unwrap().as_str().unwrap().to_string();
        let selftext = json.get("selftext").unwrap().as_str().unwrap().to_string();
        
        if selftext.len() < 10 {
            return None;
        }
        
        let thread = Thread {
            id,
            selftext,
        };
        
        Some(thread)
    } else {
        dbg!("No 'num_comments' field found");
        None
    }
}

fn handle_comment(json: Value) -> Option<Comment> {
    if let Some(body) = json.get("body") {
        let body = body.as_str().unwrap();
        let id = json.get("id").unwrap_or_else(|| {
            eprintln!("Error: 'id' is missing. JSON: {:?}", json);
            panic!("'id' is missing");
        }).as_str().unwrap_or_else(|| {
            eprintln!("Error: 'id' is not a string. JSON: {:?}", json);
            panic!("'id' is not a string");
        });
        let parent_id = json.get("parent_id").unwrap().as_str().unwrap();
        let comment = Comment {
            id: id.to_string(),
            body: body.to_string(),
            parent_id: parent_id.to_string(),
        };
        Some(comment)
    } else {
        dbg!("No 'body' field found");
        return None;
    }
}