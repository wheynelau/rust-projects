use std::fs::File;
use std::io::{BufRead, BufReader};
use zstd::Decoder;
use serde_json::Value;
use petgraph::graph::{NodeIndex};
use rayon::prelude::*;


mod threadgraph;
enum Reddit {
    Thread,
    Comment,
}

struct Thread {
    id: String,
    selftext: String,
}
struct Comment {
    id: String,
    body: String,
    parent_id: String,
}
fn main() -> std::io::Result<()> {
    let thread_file = File::open("data/askSingapore_submissions.zst")?;
    let comment_file = File::open("data/askSingapore_comments.zst")?;
    let reader = create_file_buffer(thread_file);
    let mut thread_graph = threadgraph::ThreadGraph::new();
    // just print the first 10 lines
    let _ = thread_graph.add_node("root".to_string());
    for (index, line) in reader.lines().enumerate() {
        if index == 10 {
            break;
        }   
        let line = line.unwrap();
        let json: Value = serde_json::from_str(&line).unwrap();
        let thread = create_thread(json);
        if let Some(thread) = thread {
            thread_graph.add_threads(thread.id.to_string());
            thread_graph.add_edge("root".to_string(), thread.id, );
        }

    }
    let reader = create_file_buffer(comment_file);
    for (index, line) in reader.lines().enumerate() {
        if index == 20 {
            break;
        }
        let line = line.unwrap();
        let json: Value = serde_json::from_str(&line).unwrap();
        let comment = handle_comment(json);
        if let Some(comment) = comment {
            // don't add if node not in map
            if !thread_graph.is_in_map(&comment.parent_id) {
                continue;
            }
            thread_graph.add_node(comment.id.to_string());
            thread_graph.add_edge(comment.parent_id,comment.id);

        }
    }

    thread_graph.tranverse();
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
        
        let id = json.get("name").unwrap().as_str().unwrap().to_string();
        let selftext = json.get("selftext").unwrap().as_str().unwrap().to_string();
        
        println!("{} ", id);
        
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
        let id = json.get("name").unwrap().as_str().unwrap();
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