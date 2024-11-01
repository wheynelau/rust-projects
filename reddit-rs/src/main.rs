use std::fs::File;
use std::io::{BufRead, BufReader};
use zstd::Decoder;
use serde_json::Value;
use rayon::prelude::*;
use clap::Parser;

pub mod threadgraph;
pub mod reddit;
pub mod utils;
pub mod args;
pub mod globals;
use reddit::Reddit;
use args::Cli;


fn main() -> std::io::Result<()> {

    let args = Cli::parse();
    let prefix = &args.prefix;
    let tokenizer = &args.tokenizer;
    if tokenizer.is_some() {
        let tokenizer_name = tokenizer.as_ref().unwrap();
        globals::init_tokenizer(tokenizer_name); 
    }
    let output = args.get_output();
    let thread_file = File::open(format!("{}_submissions.zst", prefix))?;
    let comment_file = File::open(format!("{}_comments.zst", prefix))?;
    let reader = create_file_buffer(thread_file);
    let mut thread_graph = threadgraph::ThreadGraph::new();
    let mut threads: Vec<Reddit> = Vec::new();

    reader.lines()
        // .take(MAX_THREADS)
        .for_each(|line| {
            // Use match or if let to handle Result more gracefully
            if let Ok(line) = line {
                // Parse JSON with error handling
                if let Ok(json) = serde_json::from_str::<Value>(&line) {
                    // Move thread directly into the if let
                    if let Some(thread) = Reddit::from_thread(json) {
                        // dbg!(&thread.id);
                        thread_graph.add_threads(&thread.id);
                        thread_graph.add_node(&thread.id);
                        threads.push(thread); // thread is moved here
                    }
                }
            }
        });

    let reader = create_file_buffer(comment_file);
    let lines: Vec<Reddit> = reader.lines()
    .par_bridge()
    .filter_map(|line| {
        let line = match line {
            Ok(line) => line,
            Err(_) => return None,
        };

        let json = match serde_json::from_str::<Value>(&line) {
            Ok(json) => json,
            Err(_) => return None,
        };

        Reddit::from_comment(json)
    })
    .collect();

    println!("Number of comments: {}", lines.len());

    for comment in lines {
        if let Some(parent_id) = &comment.parent_id {
            if thread_graph.is_in_map(parent_id) {
                thread_graph.add_node(&comment.id);
                thread_graph.add_edge(parent_id, &comment.id);
                threads.push(comment);
            }
        }
    }

    thread_graph.tranverse(threads, Some(&output));
    Ok(())
}

fn create_file_buffer(file: File) -> BufReader<zstd::Decoder<'static, BufReader<File>>> {
    let decoder = Decoder::new(file).unwrap();
    BufReader::new(decoder)
}