use std::fs::{OpenOptions, File};
use std::io::{BufWriter, Write};
use serde::{Deserialize, Serialize};

pub struct JsonlWriter {
    writer: BufWriter<File>,
}
#[derive(Serialize, Deserialize)]
pub struct JsonEntry {
    pub raw_content: String,
    pub length: usize,
}


impl JsonlWriter {
    pub fn new(filename: &str) -> std::io::Result<Self> {

        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(filename)?;
        
        Ok(JsonlWriter {
            writer: BufWriter::new(file),
        })
    }

    pub fn write_line(&mut self, content: JsonEntry) -> std::io::Result<()> {

        let json = serde_json::to_string(&content).unwrap();
        writeln!(self.writer, "{}", json)?;
        Ok(())
    }

    pub fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}