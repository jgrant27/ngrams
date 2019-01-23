use std::io::{self, BufRead, BufReader};
use std::collections::{BTreeMap};
use std::fs::File;


const NGRAM_LEN: usize = 3;
const TOP_LIMIT: usize = 100;


pub fn clean_word(word: &str) -> String {
    word.chars().filter(|ch| ch.is_alphabetic())
        .collect::<String>().to_uppercase()
}

pub fn get_ngram_counts<'a>(words: &'a Vec<String>, n: usize)
                            -> Vec<(&'a[String], usize)> {
    let mut counts: BTreeMap<&[String], usize> = BTreeMap::new();
    for ngram in words.windows(n) {
        *counts.entry(ngram).or_insert(0) += 1;
    }
    use std::iter::FromIterator;
    let mut sorted = Vec::from_iter(counts);
    sorted.sort_by(|&(_, c1), &(_, c2)| c2.cmp(&c1));
    sorted
}

pub fn from_stdin() -> String {
    let stdin = io::stdin();
    let words = stdin.lock().lines().flat_map(|line| {
        line.unwrap().split_whitespace()
            .map(|word| clean_word(word))
            .filter(|word| !word.is_empty())
            .collect::<Vec<String>>()
    }).collect::<Vec<String>>();
    let ngram_counts = get_ngram_counts(&words, NGRAM_LEN);
    let mut res: String = String::new();
    for (ngram, count) in ngram_counts.into_iter().take(TOP_LIMIT) {
        res.push_str(format!("{:?} {}\n", ngram, count).as_str());
    }
    res
}

pub fn from_path(path: &str) -> String {
    let file = File::open(path).unwrap();
    let words = BufReader::new(file).lines().flat_map(|line| {
        line.unwrap().split_whitespace()
            .map(|word| clean_word(word))
            .filter(|word| !word.is_empty())
            .collect::<Vec<String>>()
    }).collect::<Vec<String>>();
    let ngram_counts = get_ngram_counts(&words, NGRAM_LEN);
    let mut res: String = String::new();
    for (ngram, count) in ngram_counts.into_iter().take(TOP_LIMIT) {
        res.push_str(format!("{:?} {}\n", ngram, count).as_str());
    }
    res
}
