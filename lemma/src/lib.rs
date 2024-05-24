//! # Lemma
//! 
//! `lemma` is a Python library for lemmatizing text using Rust.
//! 
//! ## Installation
//! 
//! ```bash
//! # Currently the library is not on PyPI
//! pip install -e .
//! ```

use pyo3::prelude::*;
use rayon::prelude::*;
use std::{collections::{BTreeMap, BTreeSet}, sync::Arc};
mod utils;

#[pyclass]
struct Lemma {
    stopwords: BTreeSet<String>,
    lemmas: BTreeMap<String, String>,
}
#[pymethods]
impl Lemma {
    /// Creates a new instance of `Lemma`.
    /// 
    /// # Examples
    /// 
    /// ```python
    /// 
    /// from lemma import Lemma
    ///     
    /// lemma = Lemma()
    /// 
    /// ```
    #[new]
    fn new() -> Self {

        let stopwords = utils::load_stopwords();
        let lemmas = utils::load_lemmas();
        Lemma { stopwords, lemmas }
    }
    /// Returns the lemma of the input string.
    /// 
    /// # Arguments
    /// 
    /// * `input` - A string slice that holds the input string.
    /// 
    /// # Examples
    /// 
    /// ```python
    /// 
    /// from lemma import Lemma
    /// 
    /// lemma = Lemma()
    /// 
    /// print(lemma("I am going to the market"))
    /// 
    /// ```
    fn __call__(&self, input: &str) -> PyResult<String> {
        let words = utils::handle_input_str(input, &self.stopwords);
        let results: Vec<String> = words.iter()
            .map(|word| self.lemmas.get(word.as_str())
            .map_or_else(|| word.to_string(), |lemma| lemma.to_string()))
            .collect();
        Ok(results.join(" "))
    }
    /// Returns the lemma of the input string with multiple thread
    /// 
    /// # Arguments
    /// 
    /// * `input` - A list of string slices that holds the input strings.
    /// 
    /// # Examples
    /// 
    /// ```python
    /// 
    /// from lemma import Lemma
    /// 
    /// lemma = Lemma()
    /// 
    /// print(lemma.multi(["LONG TEXT 1", "LONG TEXT 2"]))
    /// 
    /// ```
    fn multi(&self, input: Vec<String>) -> PyResult<Vec<String>> {
        let lemmas = Arc::new(&self.lemmas);
        let stopwords = Arc::new(&self.stopwords);
        let results: Vec<_> = input.par_iter()
           .map(|input_str| {
                let words = utils::handle_input_str(input_str, &stopwords);
                words.iter()
                    .filter_map(|word| {
                        lemmas.get(word.as_str())
                            .map_or(Some(word.as_str()), |lemma| Some(lemma.as_str()))
                    })
                    .collect::<Vec<&str>>()
                    .join(" ")
            })
           .collect();
    
        Ok(results)
    }
}

#[pymodule]
fn lemma(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Lemma>()?;
    Ok(())
}