use clap::Parser;
use rayon::prelude::*;
use std::fs::{self};
use std::io::{self, BufRead, BufReader};
use std::{
    fs::File,
    path::{Path, PathBuf},
    sync::{atomic::AtomicUsize, Arc},
};

pub mod args;
pub mod globals;
pub mod graph;
pub mod thread;
pub mod utils;

use utils::writer::ThreadPost;

/// Creates a Vector of BTreeMap for the JSONL file
fn create_thread_posts(
    _forum_id: &str,
    threads: Vec<(String, Vec<String>)>,
    use_sentencepiece: bool,
    forum_name: String,
) -> Vec<ThreadPost> {
    // forum_id is not used, but just for compatibility

    // allocation takes time, so we only par_iter if size > 100
    threads
        .par_iter()
        .with_min_len(100)
        .map(|(thread_id, content)| {
            utils::processing::process(
                thread_id.to_string(),
                content.to_vec(),
                forum_name.to_string(),
                use_sentencepiece,
            )
        })
        .collect()
}
///
/// Handles one folder at a time
///
fn get_threads(path: &str) -> Vec<(String, Vec<String>)> {
    let entries = utils::file::single_folder(path);
    let mut threadgraph = graph::ThreadGraph::new();
    let mut threads: Vec<thread::Post> = Vec::new();
    let mut comments: Vec<thread::Post> = Vec::with_capacity(10000);
    // this shouldn't be parallelized for safety
    for entry in entries.iter() {
        let fp = File::open(entry).unwrap();

        let reader = BufReader::new(fp);

        reader.lines().for_each(|line| {
            if let Ok(line) = line {
                let json: thread::JsonStruct = serde_json::from_str(&line).unwrap();
                if let Some(thread) = thread::Post::from_json_struct(json) {
                    let thread_node = threadgraph.add_node(thread.clone());
                    if thread.is_thread {
                        threadgraph.add_threads(thread_node);
                    } else {
                        comments.push(thread);
                    };
                }
            }
        });
    }
    // add edges
    for comment in comments.iter() {
        threadgraph.add_edge(&comment.parent_post_id, &comment.id);
        threads.push(comment.clone());
    }
    println!("threads.len: {}, path: {}", threads.len(), path);
    threadgraph.tranverse()
}

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

    let start_time = std::time::Instant::now();

    if !args.safe {
        fs::create_dir_all(&out_folder).expect("Unable to create dir");
        println!("Folder has been created at `{}`", &out_folder)
    } else {
        let entries = fs::read_dir(&out_folder)
            .unwrap()
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, io::Error>>()
            .unwrap();
        if !entries.is_empty() {
            panic!("Output folder is not empty, you can run with `--safe false` to overwrite the files.");
        }
    }

    // let folder = "reddit-graph/test_main_folder/";
    // let out_folder : &str = "./output/";
    let all_folders: Vec<PathBuf> = utils::file::all_folders(&folder).unwrap();
    let total_folders = all_folders.len();

    let counter = Arc::new(AtomicUsize::new(0));
    all_folders.iter().for_each(|folder| {
        let folder = folder.to_str().unwrap();
        let forum_id = folder.split('/').last().unwrap();
        let threads: Vec<(String, Vec<String>)> = get_threads(folder);

        let posts: Vec<ThreadPost> =
            create_thread_posts(forum_id, threads, use_sentencepiece, source.clone());

        if !posts.is_empty() {
            let output_file: PathBuf = Path::new(&out_folder).join(format!("{}.jsonl", forum_id));
            utils::writer::write_jsonl(posts, output_file).unwrap();
        }

        let count = counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        print!(
            "\rProcessed {}/{} folders. Current duration: {:.2}s",
            count,
            total_folders,
            start_time.elapsed().as_secs()
        );
    });

    println!("\nAll done! Time taken: {:.3?}", start_time.elapsed());
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenizer() {
        let encoding = globals::TOKENIZER
            .get_or_init(|| {
                tokenizers::Tokenizer::from_pretrained("google/gemma-2-2b", None)
                    .expect("Unable to download tokenizer")
            })
            .encode("Hey there!", false)
            .unwrap();
        println!("{:?}", encoding.len());
    }
    #[test]
    fn test_path() {
        let initial_path = Path::new("forum_folder/output/something.jsonl");
        let folder = initial_path.parent().unwrap().to_str().unwrap();
        let stem = initial_path.file_stem().unwrap().to_str().unwrap();
        let extension = initial_path.extension().unwrap().to_str().unwrap();
        let new_file = format!("{}/{}_new.{}", folder, stem, extension);
        assert_eq!(new_file, "forum_folder/output/something_new.jsonl");
    }
    // #[test]
    // fn test_functional() {
    //     /// This needs to have a folder with the test file
    //     let file = "test_data/randomized_data.jsonl";
    //     // check if file exists

    // }
}
