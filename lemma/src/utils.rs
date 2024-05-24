use std::{collections::{BTreeMap,BTreeSet}, fs::File, io::{BufRead, Write}, path::Path};
use regex::Regex;
use reqwest::blocking::get;

type Lemmatizer = BTreeMap<String, String>;

pub fn handle_input_str(input:&str, stopwords:&BTreeSet<String>) -> Vec<String> {
    let input = input.to_lowercase();
    let word_regex: Regex = Regex::new(r"\b\w+('|-\w+)?\b").expect("Invalid regex");
    let words: Vec<String> = word_regex.find_iter(&input)
        .map(|mat| mat.as_str().to_string())
        .filter(|word| !stopwords.contains(word.as_str()))
        .collect();
    println!("{:?}", words);
    words
}

fn download_file(url: &str, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let response = get(url)?;
    let mut file = File::create(file_path)?;
    file.write_all(&response.bytes()?)?;
    Ok(())
}

pub fn load_stopwords() -> BTreeSet<String> {

    let stopwords_path = "stopwords.txt";
    let stopwords_url = "https://gist.githubusercontent.com/sebleier/554280/raw/7e0e4a1ce04c2bb7bd41089c9821dbcf6d0c786c/NLTK's%2520list%2520of%2520english%2520stopwords";

    if !Path::new(stopwords_path).exists() {
        println!("Downloading stopwords...");
        let _ = download_file(stopwords_url, stopwords_path);
    }
    let file = File::open(stopwords_path).expect("File not found");
    let stopwords: BTreeSet<String> = std::io::BufReader::new(file)
        .lines()
        .map_while(Result::ok)
        .collect();
    stopwords
}

fn parse_line(line: &str) -> Option<BTreeMap<String,String>>{

    if line.starts_with(' ') {
        return None
    }
    let parts: Vec<&str> = line.split(' ').collect();
    let base_form = parts[4].to_string().to_lowercase(); // Assuming the first word listed is the base form

    let mut lemmas: BTreeMap<String, String> = BTreeMap::new();
    if let Some(at_pos) = parts.iter().position(|&x| x == "@") {
        for &word_info in &parts[4..at_pos-1] {
            match word_info {
                "0" => continue, // Skip if the element is "0"
                word => {
                    // Assume it's a word if it's not "0" or "@"
                    lemmas.insert(word.to_string(), base_form.clone());
                }
            }
        }
    }
    Some(lemmas)
}

pub fn load_lemmas() -> Lemmatizer {
    let file = File::open("WNdb/data.noun").expect("File not found");

    let mut lemmas: Lemmatizer = BTreeMap::<String, String>::new();
    
    for line in std::io::BufReader::new(file)
                        .lines()
                        .map_while(Result::ok) {
        if let Some(parsed_lemmas) = parse_line(&line) {
            lemmas.extend(parsed_lemmas);
        }
    }
    lemmas
}
fn main() {
    let stopwords: BTreeSet<String> = load_stopwords();
    let lemmas: BTreeMap<String, String> = load_lemmas();
    let input: &str = "I am a student";

    let words: Vec<String> = handle_input_str(input, &stopwords);

    for word in words {
        if let Some(lemmas) = lemmas.get(&word) {
                println!("{:?}", lemmas);
            }
        }
    }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let line: &str = "00113113 04 n 03 press 0 pressure 0 pressing 0 012 @ 00112312 n 0000 + 01387786 v 0306 + 01390616 v 0301 + 01447257 v 0301 + 01447257 v 0201 + 01447632 v 0101 + 01754105 v 0101 + 01387786 v 0106 + 01390616 v 0101 + 01447257 v 0101 ~ 00113532 n 0000 ~ 00356790 n 0000 ";
        let lemmas: BTreeMap<String, String> = parse_line(line).unwrap();
        println!("{:?}", lemmas);
        assert_eq!(lemmas.len(), 3);
    }
}