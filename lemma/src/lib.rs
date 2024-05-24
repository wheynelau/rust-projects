use pyo3::prelude::*;
use rayon::prelude::*;
use std::collections::{BTreeSet, BTreeMap};

mod utils;

#[pyclass]
struct Lemma {
    stopwords: BTreeSet<String>,
    lemmas: BTreeMap<String, String>,
}
/// Formats the sum of two numbers as string.
#[pymethods]
impl Lemma {
    #[new]
    fn new() -> Self {
        let stopwords = utils::load_stopwords();
        let lemmas = utils::load_lemmas();
        Lemma { stopwords, lemmas }
    }

    fn __call__(&self, input: &str) -> PyResult<Vec<String>> {
        let words = utils::handle_input_str(input, &self.stopwords);
        let results: Vec<_> = words.iter()
        .filter_map(|word| self.lemmas.get(word.as_str()))
        .cloned()
        .collect();
        Ok(results)
    }
    fn multi(&self, input: Vec<String>) -> PyResult<Vec<Vec<String>>> {
        let results: Vec<_> = input.par_iter()
           .map(|input_str| {
                let words = utils::handle_input_str(input_str, &self.stopwords);
                words.iter()
                   .filter_map(|word| self.lemmas.get(word.as_str())) // Convert &std::string::String to &str
                   .cloned()
                   .collect::<Vec<String>>()
            })
           .collect();
    
        Ok(results)
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn lemma(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Lemma>()?;
    Ok(())
}