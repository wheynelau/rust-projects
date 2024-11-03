// src/globals.rs
use std::sync::OnceLock;
use tokenizers;

/// The global tokenizer
/// 
/// This is a `OnceLock<tokenizers::Tokenizer>` that will be initialized when called with 
/// `get_or_init` and a closure that returns a `tokenizers::Tokenizer`
/// 
/// # Example
/// 
/// ```
/// pub mod globals;
/// 
/// globals::TOKENIZER.get_or_init(|| {
///    tokenizers::Tokenizer::from_pretrained("openai-community/gpt2", None).unwrap()
/// });
/// 
/// ```
static TOKENIZER: OnceLock<tokenizers::Tokenizer> = OnceLock::new();
pub static RE: OnceLock<regex::Regex> = OnceLock::new();
pub static RE2: OnceLock<regex::Regex> = OnceLock::new();

/// Initialize the regex
///
/// This should be called at the beginning of the program
///
/// # Example
/// ```
/// pub mod globals;
/// globals::init_regex();
///
/// // Continue with the program
///
/// ```
pub fn init_regex() {
    RE.get_or_init(|| {
        regex::Regex::new(r"-{2,}|={2,}|http\S+|(?:[\w\.-]+)?@\S+|#\S+|\s{2,}").unwrap()
    });
    RE2.get_or_init(|| regex::Regex::new(r"\s+").unwrap());
}

/// Helper function to initialize the tokenizer
///
/// This may be called at the beginning of the program if choosing to use a specific tokenizer
///
/// # Arguments
///
/// * `tokenizer_name` - `&String` - The name of the tokenizer to use, this should be in the format of `huggingface <org>/<name>` or a path to a tokenizer.json file
///
/// # Example
///
/// ```
/// pub mod globals;
///
/// globals::init_tokenizer(&"openai-community/gpt2".to_string());
///
/// // Continue with the program
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

/// Helper function to tokenize directly
///
/// This function will tokenize the content and return the encoding directly, abstracting the need to call `get` and `unwrap` on the OnceLock.
///
// # Arguments
///
/// * `content` : `&str` - The content to tokenize
///
/// # Returns
///
/// * `tokenizers::Encoding` - The tokenized content
///
/// # Example
///
/// ```
/// pub mod globals;
///
///
/// globals::init_tokenizer(&"openai-community/gpt2".to_string());
/// let content = "Hello world";
/// let encoding = globals::tokenize(content);
///
/// ```
///
/// # Panics
///
/// This function will panic if the tokenizer has not been initialized
pub fn tokenize(content: &str) -> tokenizers::Encoding {
    TOKENIZER
        .get()
        .expect("Tokenizer has not been initialized")
        .encode(content, false)
        .unwrap()
}


#[cfg(test)]
mod tokenizer_tests {
    use super::*;

    #[test]
    fn test_tokenizer() {

        init_tokenizer(&"openai-community/gpt2".to_string());
        let encoding = tokenize("Hello world");
        assert!(!encoding.get_tokens().is_empty());
    }

    #[test]
    #[should_panic (expected = "Tokenizer has not been initialized")]
    fn test_panic() {
        // Try to get the tokenizer without initializing it
        tokenize("Hello world");
    }

    #[test]
    #[should_panic]
    fn test_invalid_huggingface_name() {
        init_tokenizer(&"no_such_model".to_string());
    }

}
