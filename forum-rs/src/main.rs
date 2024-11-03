#![doc = include_str!("../README.md")]

use clap::Parser;
use rayon::prelude::*;
use std::fs::{self};
use std::io::Write;
use std::{
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        Arc,
    },
};

use std::time::{Duration, Instant};

/**

# Struct for the command line arguments

# Arguments

* `input` - The input folder containing the forum data
* `output` - The output folder where the processed data will be stored
* `tokenizer` - The tokenizer to use for tokenization
* `source` - The source of the data
* `safe` - Whether to overwrite the output folder or not

# Panics

* If the input folder does not exist
* If the output folder does not exist
* If the tokenizer is not valid
* If huggingface hub is not authorized


*/
pub mod args;

/**

# Module for the experimental functions

This module containes functions that may not produce the best performance but are experimental
*/
pub mod experimental;
pub mod forum_thread;
pub mod globals;
pub mod graph;
pub mod utils;

/// Process the folder
/// 
/// What this function does:
/// 1. Get the threads from the folder
/// 2. Create the thread posts
/// 3. Write the thread posts to a file
/// 
/// # Arguments
/// 
/// * `folder` - `&Path` - The folder containing list of `jsonl` files
/// * `out_folder` - `&String` - The output folder where the processed data will be stored
/// * `use_sentencepiece` - `&bool` - Whether to use sentencepiece for tokenization, the name does not mean that it 
/// will use sentencepiece, it will use the tokenizer specified in the `tokenizer` argument. 
/// * `source` - `&String` - The source of the data. This is just for labelling. 
/// 
/// # Example
/// 
/// ```rust
/// use std::path::Path;
/// 
/// let folder = Path::new("main_folder/sub1/");
/// let out_folder = "./output/";
/// let use_sentencepiece = true;
/// let source = "reddit".to_string();
/// process_folder(folder, &out_folder, &use_sentencepiece, &source);
/// 
/// ```
fn process_folder(folder: &Path, out_folder: &String, use_sentencepiece: &bool, source: &String) {
    // dbg!(&folder);
    let folder = folder.to_str().unwrap();
    let forum_id = folder.split('/').last().unwrap();

    let threads: Vec<(String, Vec<String>)> = experimental::sender::get_threads(folder);

    let (posts, bytes) = forum_thread::create_thread_posts(
        forum_id,
        threads,
        *use_sentencepiece,
        source.to_string(),
    );

    if !posts.is_empty() {
        let output_file: PathBuf = Path::new(&out_folder).join(format!("{}.jsonl", forum_id));
        utils::writer::write_jsonl(posts, bytes, output_file).unwrap();
    }
}
///
/// Entry point of the program
///
/// This function will parse the arguments and start the processing of the folders
///
/// # Arguments
///
/// * `input` - The input folder containing the forum data
/// * `output` - The output folder where the processed data will be stored
/// * `tokenizer` - The tokenizer to use for tokenization
/// * `source` - The source of the data
/// * `safe` - Whether to overwrite the output folder or not
///     
/// # Example
///
/// ```bash
/// cargo run --release -- --input "reddit-graph/test_main_folder/" --output "./output/" \
///     --tokenizer "model-name" or "path-to-tokenizer.json" --source "reddit" --safe false
/// ```
///
/// # Note
///
/// Folders must contain subfolders with the forum data
/// ```plaintext
/// main_folder/  
/// ├── sub1/  
/// │   └── *.jsonl  
/// └── sub2/  
///     └── *.jsonl  
/// ```
///
/// The output folder will contain the processed data
/// ```plaintext
/// output/
/// ├── sub1.jsonl
/// └── sub2.jsonl
/// ```
fn main() {
    let args = args::Cli::parse();
    let folder: String = args.input;
    let out_folder: String = args.output;
    let tokenizer: Option<String> = args.tokenizer;
    let source: String = args.source;
    let use_sentencepiece: bool = tokenizer.as_ref().is_some();

    if let Some(tokenizer) = tokenizer {
        globals::init_tokenizer(&tokenizer);
    }
    // For safety, the output folder is not created if not found
    // Also if not empty, it will panic.
    if !args.safe {
        fs::create_dir_all(&out_folder).expect("Unable to create dir");
        println!("Folder has been created at `{}`", &out_folder)
    } else {
        let entries = fs::read_dir(&out_folder)
            .unwrap()
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, std::io::Error>>()
            .unwrap();
        if !entries.is_empty() {
            panic!("Output folder is not empty, you can run with `--safe false` to overwrite the files.");
        }
    }

    // let folder = "reddit-graph/test_main_folder/";
    // let out_folder : &str = "./output/";
    let all_folders: Vec<PathBuf> = utils::file::all_folders(&folder).unwrap();

    // Reorder the largest size first
    // This should speed up the parallel processing
    // let all_folders = utils::file::reorder_by_size(all_folders);
    let total_folders = all_folders.len();

    // Before the par_iter loop:
    let counter = Arc::new(AtomicUsize::new(0));
    let running = Arc::new(AtomicBool::new(true));
    let running_clone = running.clone();
    let counter_clone = counter.clone();
    let start_time_clone = Instant::now();
    // Spawn progress display thread
    let progress_thread = std::thread::spawn(move || {
        while running_clone.load(Ordering::SeqCst) {
            let count = counter_clone.load(Ordering::SeqCst);
            print!(
                "\rProcessed {}/{} folders. Current duration: {:2}m {:.2}s",
                count,
                total_folders,
                start_time_clone.elapsed().as_secs() / 60,
                start_time_clone.elapsed().as_secs() % 60
            );
            std::io::stdout().flush().unwrap();
            std::thread::sleep(Duration::from_millis(500));
        }
        // One final update after completion
        let count = counter_clone.load(Ordering::SeqCst);
        println!(
            "\rProcessed {}/{} folders. Current duration: {:.2}s",
            count,
            total_folders,
            start_time_clone.elapsed().as_secs()
        );
    });

    all_folders.par_iter().for_each(|folder| {
        process_folder(folder, &out_folder, &use_sentencepiece, &source);
        counter.fetch_add(1, Ordering::SeqCst);
    });
    // After the loop completes, stop the progress thread
    running.store(false, Ordering::SeqCst);
    progress_thread.join().unwrap();
}
#[cfg(test)]
mod main_tests {
    use super::*;
    use pretty_assertions::assert_eq;
    #[test]
    fn test_path() {
        let initial_path = Path::new("forum_folder/output/something.jsonl");
        let folder = initial_path.parent().unwrap().to_str().unwrap();
        let stem = initial_path.file_stem().unwrap().to_str().unwrap();
        let extension = initial_path.extension().unwrap().to_str().unwrap();
        let new_file = format!("{}/{}_new.{}", folder, stem, extension);
        assert_eq!(new_file, "forum_folder/output/something_new.jsonl");
    }

    #[test]
    fn test_integration() {
        // this needs to have a folder with jsonl files
        globals::init_regex();
        let folder = String::from("test_data/forum_276");

        let threads: Vec<(String, Vec<String>)> = experimental::sender::get_threads(&folder);

        assert_eq!(threads.len(), 42);
    }
}
