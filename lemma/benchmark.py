import time

import pandas as pd
from lemma import Lemma
import nltk
from nltk.corpus import stopwords
from nltk.tokenize import word_tokenize
from nltk.stem import WordNetLemmatizer

import time

class Timer:

    def __init__(self,name  = "timer"):
        self.name = name

    def __enter__(self):
        self.start = time.time()
        return self  # This allows access to the instance within the context

    def __exit__(self, exc_type, exc_value, traceback):
        self.end = time.time()
        self.interval = self.end - self.start
        print(f"{self.name} took {self.interval:.3f} seconds")

def remove_stopwords_and_lemmatize(text):
    stop_words = set(stopwords.words('english'))
    
    lemmatizer = WordNetLemmatizer()

    word_tokens = word_tokenize(text.lower())
    
    filtered_and_lemmatized = [
        lemmatizer.lemmatize(word) for word in word_tokens if word.lower() not in stop_words
    ]
    
    return ' '.join(filtered_and_lemmatized)

nltk.download('stopwords')
nltk.download('wordnet')
nltk.download('omw-1.4')

lemmatizer = Lemma()

df = pd.read_csv('data/imdb_dataset.csv')[:1000]

with Timer("Rust map"):
    rust = df['review'].map(lambda x: lemmatizer(x))

with Timer("Python map"):
    py = df['review'].map(lambda x: remove_stopwords_and_lemmatize(x))

with Timer("Rust list"):
    rust_list =  lemmatizer.multi(df['review'].to_list())

assert rust_list == rust.to_list() , "Rust list and Rust map are not equal"



