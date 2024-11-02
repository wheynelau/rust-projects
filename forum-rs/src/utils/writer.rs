use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use serde::{Serialize};
use rayon::prelude::*;

/// Enum for serialization
#[derive(Serialize)]
pub struct ThreadPost {
    pub length: usize,
    pub raw_content: String,
    pub thread_id: String,
    pub source: String,
}

/// Writes a vector of ThreadPost to a JSONL file
pub fn write_jsonl(
    data: Vec<ThreadPost>,
    file_path: PathBuf,
) -> std::io::Result<()> {
    // Trying to implement rayon
    // Note that the size of the input should be checked before entering here
    let chunk_size: usize = 50000;

    let folder = file_path.parent().unwrap().to_str().unwrap();
    let stem = file_path.file_stem().unwrap().to_str().unwrap();
    let extension = file_path.extension().unwrap().to_str().unwrap();

    if data.len() <= chunk_size {
        // TODO: Possible refactor for the naming convention
        let file_path = Path::new(folder).join(format!("{}_0.{}", stem, extension));
        let file = File::create(file_path)?;
        let mut writer = BufWriter::new(file);

        for hashmap in data {
            let json_line = serde_json::to_string(&hashmap).unwrap();
            writeln!(&mut writer, "{}", json_line).unwrap();
        }
        Ok(())
    } else {
        // break the data into chunks of N chunks
        data.par_chunks(chunk_size)
            .enumerate()
            .for_each(|(i, chunk)| {
                let file_path = Path::new(folder).join(format!("{}_{}.{}", stem, i, extension));
                let file = File::create(file_path).unwrap();
                let mut writer = BufWriter::new(file);

                for hashmap in chunk {
                    let json_line = serde_json::to_string(hashmap).unwrap();
                    writeln!(&mut writer, "{}", json_line).unwrap();
                }
            });
        Ok(())
    }
}