# ngrams

A somewhat _fast_ most-common ngrams finder in [Rust](https://www.rust-lang.org).

e.g. Will find the top 100 most common tri-grams in < 1 sec for [Origin of the Species](http://www.gutenberg.org/ebooks/2009) on a circa 2008 macbook.

```
mbp2008:ngrams jgrant$ time cat text/pg2009.txt | cargo run --release 
    Finished release [optimized] target(s) in 0.07s
     Running `target/release/ngrams`
STDIN:
["OF", "THE", "SAME"] 320
["THE", "SAME", "SPECIES"] 126
["CONDITIONS", "OF", "LIFE"] 125
["IN", "THE", "SAME"] 116
["OF", "NATURAL", "SELECTION"] 107
["FROM", "EACH", "OTHER"] 103
["SPECIES", "OF", "THE"] 98
["ON", "THE", "OTHER"] 89
["THE", "OTHER", "HAND"] 81
["THE", "CASE", "OF"] 78
["THE", "THEORY", "OF"] 75
["SOME", "OF", "THE"] 73
["OF", "THE", "WORLD"] 72
["PARTS", "OF", "THE"] 72
...
["BELIEVE", "THAT", "THE"] 33
["FOR", "INSTANCE", "THE"] 33
["FROM", "THE", "SAME"] 33



real	0m0.952s
user	0m0.769s
sys	0m0.099s

``` 

## Install rust

Follow the [instructions](https://www.rust-lang.org/tools/install) here.


## Run the tests

```
mbp2008:ngrams jgrant$ cargo test --release
   Compiling ngrams v0.1.0 (/Users/jaybones/ngrams)
    Finished release [optimized] target(s) in 7.15s

running 2 tests
test ngram_tests::clean_word_test ... ok
test ngram_tests::from_path_test ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## Specifiying multiple input files

e.g. running against [Origin of the Species](http://www.gutenberg.org/ebooks/2009) and [The Adventures of Sherlock Holmes](http://www.gutenberg.org/ebooks/1661)

```
mbp2008:ngrams jgrant$ time cargo run --release text/pg2009.txt text/big.txt 
    Finished release [optimized] target(s) in 0.63s
     Running `target/release/ngrams text/pg2009.txt text/big.txt`
text/pg2009.txt:
["OF", "THE", "SAME"] 320
["THE", "SAME", "SPECIES"] 126
["CONDITIONS", "OF", "LIFE"] 125
...
["BELIEVE", "THAT", "THE"] 33
["FOR", "INSTANCE", "THE"] 33
["FROM", "THE", "SAME"] 33

text/1661.txt:
["ONE", "OF", "THE"] 49
["I", "THINK", "THAT"] 46
["IT", "IS", "A"] 46
...
["GUTENBERG", "LITERARY", "ARCHIVE"] 13
["I", "CAME", "TO"] 13
["I", "HAVE", "A"] 13


real	0m1.290s
user	0m1.078s
sys	0m0.096s
```