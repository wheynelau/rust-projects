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
The csv and only data.noun is provided in the repo just for testing.

- polars should be faster than pandas
- I expect spacy to be more optimized but I did not test it, yet.
- Also, it seems map does not perform multithreading, which makes it an unfair comparison.

### M1 Macbook Air
```
Rust map took 75.746 seconds
Python map took 92.464 seconds
Rust list took 22.032 seconds
```

### i5-11400 nproc = 12 on Debian 11 WSL
```
Rust map took 61.356 seconds
Python map took 52.979 seconds
Rust list took 10.359 seconds
```
## Issues

- The lemmatization is not working
- Most of the code in `src/main.rs` is hardcoded, such as the paths. Also, no autodownloading of the wordnet files
- The code is not idiomatic or "correct" rust code, as I am still learning
- Due to all the above, the code is not published anywhere, but it might be a good learning tool for someone


## References
```
@InProceedings{maas-EtAl:2011:ACL-HLT2011,
  author    = {Maas, Andrew L.  and  Daly, Raymond E.  and  Pham, Peter T.  and  Huang, Dan  and  Ng, Andrew Y.  and  Potts, Christopher},
  title     = {Learning Word Vectors for Sentiment Analysis},
  booktitle = {Proceedings of the 49th Annual Meeting of the Association for Computational Linguistics: Human Language Technologies},
  month     = {June},
  year      = {2011},
  address   = {Portland, Oregon, USA},
  publisher = {Association for Computational Linguistics},
  pages     = {142--150},
  url       = {http://www.aclweb.org/anthology/P11-1015}
}
```