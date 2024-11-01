// src/globals.rs
use std::sync::OnceLock;
use tokenizers;

pub static TOKENIZER: OnceLock<tokenizers::Tokenizer> = OnceLock::new();

// Helper function to initialize the tokenizer
pub fn init_tokenizer(tokenizer_name: &String) {
    if tokenizer_name.ends_with(".json") {
        println!("Loading tokenizer from file: {}", tokenizer_name);
        TOKENIZER
            .set(tokenizers::Tokenizer::from_file(tokenizer_name).unwrap())
            .expect("Unable to load tokenizer");
    } else {
        println!("Loading tokenizer: {}", tokenizer_name);
        TOKENIZER
            .set(tokenizers::Tokenizer::from_pretrained(tokenizer_name, None).unwrap())
            .expect("Unable to load tokenizer");
    }
}

pub fn get_tokenizer() -> Option<&'static tokenizers::Tokenizer> {
    TOKENIZER.get()
}

// Helper function to tokenize directly
pub fn tokenize(content:&str) -> tokenizers::Encoding {
    TOKENIZER.get()
    .unwrap()
    .encode(content, false)
    .unwrap()
}
