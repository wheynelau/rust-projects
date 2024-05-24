use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
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
        let mut results = Vec::new();
        for word in words {
            if let Some(lemma) = self.lemmas.get(&word) {
                results.push(lemma.clone());
            }
        }
        Ok(results)
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn lemma(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Lemma>()?;
    Ok(())
}