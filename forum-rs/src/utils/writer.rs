use rayon::prelude::*;
use serde::Serialize;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

// **1 = B
// **2 = KB
// **3 = MB
const MAX_BYTES_PER_FILE: usize = 100 * 1024_usize.pow(2);

/// Enum for serialization
#[derive(Serialize, Clone, Default)]
pub struct ThreadPost {
    pub length: usize,
    pub raw_content: String,
    pub thread_id: String,
    pub source: String,
}

fn get_chunk_size(bytes: usize, data: &[ThreadPost]) -> usize {
    let num_files = bytes.div_ceil(MAX_BYTES_PER_FILE);
    let num_splits = num_files.max(1);
    data.len().div_ceil(num_splits)
}

/// Writes a vector of ThreadPost to a JSONL file
pub fn _write_jsonl(data: Vec<ThreadPost>, bytes: usize, file_path: PathBuf) -> std::io::Result<()> {
    // Trying to implement rayon
    // Note that the size of the input should be checked before entering here
    let chunk_size = get_chunk_size(bytes, &data);

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

pub fn write_jsonl(data: Vec<ThreadPost>, _bytes: usize, file_path: PathBuf) -> std::io::Result<()> {


    let folder = file_path.parent().unwrap().to_str().unwrap();
    let stem = file_path.file_stem().unwrap().to_str().unwrap();
    let extension = file_path.extension().unwrap().to_str().unwrap();

    let file_path = Path::new(folder).join(format!("{}_0.{}", stem, extension));
    let file = File::create(file_path)?;
    let handle = std::thread::spawn(move || {
        let mut writer = BufWriter::new(file);

        for hashmap in data {
            let json_line = serde_json::to_string(&hashmap).unwrap();
            writeln!(&mut writer, "{}", json_line).unwrap();
        }
    });
    handle.join().unwrap();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    #[test]
    fn test_get_chunk_size() {
        let post = ThreadPost::default();
        // repeat 1000 for data
        let data: Vec<ThreadPost> = vec![post; 1000];
        // Total size 300MB
        let bytes = 300 * 1024_usize.pow(2);
        let chunk_size = super::get_chunk_size(bytes, &data);
        // 1000 / 3 ceil = 334
        assert_eq!(chunk_size, 334);
    }
}
