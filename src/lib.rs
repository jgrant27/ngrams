use std::io::{self, BufRead, BufReader};
use std::collections::{HashMap};
use std::fs::File;
use std::iter::FromIterator;
use std::fmt::Debug;

const NGRAM_LEN: usize = 3;
const TOP_LIMIT: usize = 100;
const SPACE: char = ' ';

pub fn load_words<T: Debug, R: Iterator<Item=Result<String, T>>>(lines: R) -> String {
    let mut nbuf = String::new();
    lines.for_each(|line| {
        line.unwrap().split_whitespace().for_each(|word| {
            let mut cword = word.chars().filter(|ch| ch.is_alphabetic())
                .collect::<String>().to_uppercase();
            cword.push(SPACE);
            if !cword.is_empty() {
                nbuf.push_str(&cword);
            }
        });
    });
    dbg!(nbuf.len());
    nbuf
}

pub fn get_ngram_counts<'a>(nbuf: &'a String) -> Vec<(&'a str, usize)> {
    let mut cmap: HashMap<&str, usize> = HashMap::new();
    let ixs = nbuf.match_indices(SPACE).map(|(i, _)| i).collect::<Vec<usize>>();
    dbg!(ixs.len());
    for (n, _) in ixs.iter().enumerate() {
        let start = ixs[n] + 1;
        if n + NGRAM_LEN < ixs.len() - 1 {
            let end = ixs[n + NGRAM_LEN];
            let ngram = &nbuf[start..end];
            *cmap.entry(&ngram).or_insert(0) += 1;
        }
    }
    let mut ngram_counts = Vec::from_iter(cmap);
    ngram_counts.sort_by(|&(s1, _), &(s2, _)| s1.cmp(&s2));
    ngram_counts.sort_by(|&(_, c1), &(_, c2)| c2.cmp(&c1));
    ngram_counts
}

pub fn get_top_counts<'a>(ngram_counts: &'a Vec<(&'a str, usize)>) -> String {
    let mut res: String = String::new();
    for (ngram, count) in ngram_counts.into_iter().take(TOP_LIMIT) {
        res.push_str(format!("{:?} {}\n", ngram, count).as_str());
    }
    res
}

pub fn from_path(path: &str) -> String {
    let file = File::open(path).unwrap();
    let mut lines = BufReader::new(file).lines();

    let nbuf = load_words(&mut lines);
    let ngram_counts = get_ngram_counts(&nbuf);
    get_top_counts(&ngram_counts)
}

pub fn from_stdin() -> String {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let nbuf = load_words(&mut lines);
    let ngram_counts = get_ngram_counts(&nbuf);
    get_top_counts(&ngram_counts)
}
