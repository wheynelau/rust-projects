use serde::{Deserialize, Serialize};
#[allow(dead_code)]
#[allow(unused_variables)]
use std::{env, fs, io, path};

use clap::{Parser, Subcommand};
use std::collections::BTreeMap;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    trash_path: String,
    original_path: String,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Put files in trash
    Put {
        /// Recursive flag
        #[clap(short = 'r', long, default_value = "false")]
        recursive: bool,

        /// Individual files and directories
        #[clap(name = "FILE")]
        files: String,
    },
    /// List trashed files
    List,
    /// Restore trashed files
    Restore {
        /// Restore all files
        #[clap(short = 'a', long, default_value = "false")]
        all: bool,

        /// Individual files and directories
        #[clap(name = "FILE")]
        files: Option<String>,
    },
    /// Undo last trashed file
    Undo,
}

fn load_json(trash_dir: &str) -> BTreeMap<String, String> {
    let json_file = format!("{}/trash.json", trash_dir);
    let file = fs::File::open(&json_file).unwrap_or_else(|_| fs::File::create(&json_file).unwrap());
    let reader = io::BufReader::new(file);
    let data: BTreeMap<String, String> = serde_json::from_reader(reader).unwrap_or_default();
    data
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();
    let trash_dir = env::var("TRASH_DIR").unwrap_or_else(|_| {
        let home = env::var("HOME").unwrap();
        format!("{}/.trash_rs", home)
    });
    let mut trash_info = load_json(&trash_dir);
    // Check if trash directory exists
    if !fs::metadata(&trash_dir).is_ok() {
        fs::create_dir(&trash_dir).unwrap();
    }
    match &cli.command {
        Commands::Put { recursive, files } => {
            let file = fs::metadata(files).unwrap_or_else(|_| {
                eprintln!("{} does not exist", files);
                std::process::exit(1);
            });
            if file.is_dir() && !recursive {
                eprintln!("{} is a directory, use -r flag to delete", files);
                std::process::exit(1);
            } else if file.is_dir() && *recursive {
                // get the root folder name instead of full path
                let root_folder = files.trim_end_matches('/').split("/").last().unwrap();
                // create the target folder in trash
                let target_folder = format!("{}/{}", trash_dir, root_folder);

                // shift the items into trash by using rename and just changing the root
                // For example:
                // /home/user/to_trash.txt -> /home/user/.trash_rs/to_trash.txt
                // /home/user/to_trash_folder -> /home/user/.trash_rs/to_trash_folder

                fs::rename(files, target_folder)
            } else {
                let root_file = files.trim_end_matches('/').split("/").last().unwrap();

                fs::rename(files, format!("{}/{}", trash_dir, root_file))?;
                // add the file and original path to hashmap
                trash_info.insert(root_file.to_string(), files.to_string());

                Ok(())
            }
        }
        Commands::List => {
            let files = fs::read_dir(trash_dir).unwrap();
            for file in files {
                let file = file.unwrap().file_name();
                let file = file.to_str().unwrap();
                if file == "trash.json" {
                    continue;
                }
                println!("{}", file);
            }
            Ok(())
        }
        Commands::Restore { all: bool, files } => {
            let files = fs::read_dir(trash_dir);

            Ok(())
        }
        Commands::Undo => {
            let last_file = match trash_info.iter().last() {
                Some(file) => file,
                None => {
                    eprintln!("No files to undo");
                    std::process::exit(1);
                }
            };
            fs::rename(format!("{}/{}", trash_dir, last_file.0), last_file.0)
        }
    }
}
// fn put(file: &str) -> io::Result<()> {
//     let mut file = fs::File::create(file)?;
//     fs::rename(file, file)?;

//     Ok(())
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recursive() {
        let walker =
            WalkDir::new("/Users/waynelau/coding/rust-projects/trash/test_dir").into_iter();
        for entry in walker {
            dbg!(entry.unwrap());
        }
        let walker = WalkDir::new("test_dir").into_iter();
        for entry in walker {
            dbg!(path::absolute(entry.unwrap().path())?);
        }
    }
    #[test]
    fn test_split() {
        let path = "/home/wayne/test_dir/";
        // remove trailing slash
        let path = path.trim_end_matches('/');
        let root_folder = path.split("/").last().unwrap();
        dbg!(root_folder);
    }
}
