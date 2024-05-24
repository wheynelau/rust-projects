# Making lemmatization go BRRR (i think)

THIS IS NOT WORKING, but it's pretty cool

## Learning points

String manipulation in rust:
- `&str` vs `String`
- understanding `map` and `collect`
- File processing with `std::fs::File`
- Multithreading with `rayon` crate
- Some rust docs

## Setup

Most of the filepaths are hardcoded, so you need to download the files

Wordnet db should be unzipped to `./WNdb/` folder and the kaggle dataset to `data/imdb_dataset.csv`

## Installation

```bash

git clone <this repo>

conda create -n lemma python=3.11

conda activate lemma

pip install -e . 

```

If you're interested in documentation like every great developer, you can run `cargo doc --open --document-private-items --lib` to see the docs for Lemma.  
Not sure if there's a better way to do this.  

_Note_: Editable mode might not be needed

## How to run

The demonstration is in `benchmark.py`. 
Please note that the lemmatization is not working the same as the python version. And further investigation is needed.  
The csv file is from the [kaggle dataset](https://www.kaggle.com/datasets/lakshmi25npathi/imdb-dataset-of-50k-movie-reviews).

## Benchmark results

- On a M1 Macbook Air.
- polars should be faster than pandas
- I expect spacy to be more optimized but I did not test it, yet.
- Also, it seems map does not perform multithreading, which makes it an unfair comparison.

```
Rust map took 75.746 seconds
Python map took 92.464 seconds
Rust list took 22.032 seconds
```

## Issues

- The lemmatization is not working
- Most of the code in `src/main.rs` is hardcoded, such as the paths. Also, no autodownloading of the wordnet files
- The code is not idiomatic or "correct" rust code, as I am still learning
- Due to all the above, the code is not published anywhere, but it might be a good learning tool for someone
  