use clap::Parser;
use rayon::prelude::*;
use std::fs::{self};
use std::io::{self, BufRead, BufReader, Write};
use std::{
    fs::File,
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        Arc,
    },
};

use std::time::{Duration, Instant};
pub mod args;
pub mod experimental;
pub mod forum_thread;
pub mod globals;
pub mod graph;
pub mod utils;

use graph::ThreadGraph;
use utils::writer::ThreadPost;

/// Creates a Vector of BTreeMap for the JSONL file
fn create_thread_posts(
    _forum_id: &str,
    threads: Vec<(String, Vec<String>)>,
    use_sentencepiece: bool,
    forum_name: String,
) -> (Vec<ThreadPost>, usize) {
    let byte_counter = AtomicUsize::new(0);
    let posts = if threads.len() > 5000 {
        // Parallel processing for large number of threads
        let mut posts: Vec<ThreadPost> = Vec::with_capacity(threads.len());
        threads
            .par_iter()
            .map(|(thread_id, content)| {
                let threadpost = utils::processing::process(
                    thread_id.to_string(),
                    content.to_vec(),
                    forum_name.to_string(),
                    use_sentencepiece,
                );
                byte_counter.fetch_add(threadpost.raw_content.len(), Ordering::Relaxed);
                threadpost
            })
            .collect_into_vec(&mut posts);
        posts
    } else {
        // Sequential processing for smaller number of threads
        let posts: Vec<ThreadPost> = threads
            .iter()
            .map(|(thread_id, content)| {
                let threadpost = utils::processing::process(
                    thread_id.to_string(),
                    content.to_vec(),
                    forum_name.to_string(),
                    use_sentencepiece,
                );
                byte_counter.fetch_add(threadpost.raw_content.len(), Ordering::Relaxed);
                threadpost
            })
            .collect();
        posts
    };

    (posts, byte_counter.into_inner())
}

#[allow(dead_code)]
fn get_threads(path: &str) -> Vec<(String, Vec<String>)> {
    let entries = utils::file::single_folder(path);
    let mut threadgraph = ThreadGraph::new();
    let mut comments: Vec<forum_thread::Post> = Vec::with_capacity(10000);

    // let loop_start = std::time::Instant::now();
    // this shouldn't be parallelized for safety
    entries.iter().for_each(|entry| {
        let fp = File::open(entry).unwrap();
        let reader = BufReader::new(fp);
        let threads: Vec<forum_thread::Post> = reader
            .lines()
            .par_bridge()
            .filter_map(|line| line.ok())
            .filter_map(|line| {
                serde_json::from_str::<forum_thread::JsonStruct>(&line)
                    .ok()
                    .and_then(forum_thread::Post::from_json_struct)
            })
            .collect();

        for thread in threads {
            let thread_node = threadgraph.add_node(thread.clone());
            match thread.is_thread {
                true => threadgraph.add_threads(thread_node),
                false => comments.push(thread),
            }
        }
    });

    //println!("Time taken for loop: {:.2?}", loop_start.elapsed());
    // add edges
    // let comments = comments.lock().unwrap();
    // let mut threadgraph = threadgraph.lock().unwrap();
    // let comment_time = std::time::Instant::now();
    for comment in comments.iter() {
        threadgraph.add_edge(&comment.parent_post_id, &comment.id);
    }
    // println!("Time taken for comments: {:.2?}", comment_time.elapsed());
    // let traverse_time = std::time::Instant::now();
    threadgraph.traverse()
    // println!("Time taken for traverse: {:.2?}", traverse_time.elapsed());
    // threads
}

fn process_folder(
    folder: &PathBuf,
    out_folder: &String,
    use_sentencepiece: &bool,
    source: &String,
) {
    // dbg!(&folder);
    let folder = folder.to_str().unwrap();
    let forum_id = folder.split('/').last().unwrap();

    let threads: Vec<(String, Vec<String>)> = experimental::sender::get_threads(folder);

    let (posts, bytes) =
        create_thread_posts(forum_id, threads, *use_sentencepiece, source.to_string());

    if !posts.is_empty() {
        let output_file: PathBuf = Path::new(&out_folder).join(format!("{}.jsonl", forum_id));
        utils::writer::write_jsonl(posts, bytes, output_file).unwrap();
    }
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
                "\rProcessed {}/{} folders. Current duration: {:.2}s",
                count,
                total_folders,
                start_time_clone.elapsed().as_secs()
            );
            std::io::stdout().flush().unwrap();
            std::thread::sleep(Duration::from_millis(100));
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
        process_folder(&folder, &out_folder, &use_sentencepiece, &source);
        counter.fetch_add(1, Ordering::SeqCst);
    });
    // After the loop completes, stop the progress thread
    running.store(false, Ordering::SeqCst);
    progress_thread.join().unwrap();
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

}
