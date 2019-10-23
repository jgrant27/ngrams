use std::collections::HashMap;
use std::fs::File;
use std::iter::FromIterator;

use memmap::{Mmap, MmapOptions};

const NGRAM_LEN: usize = 3;
const TOP_LIMIT: usize = 100;
const NEW_LINE: char = '\n';
const CARRIAGE_RETURN: char = '\r';

pub fn load_word_indexes(mmap: &Mmap) -> Vec<usize> {
    mmap.iter()
        .map(|&byte| byte as char)
        .enumerate()
        .fold(Vec::new(), |mut v, (ind, ch)| {
            if (ch.is_whitespace() || NEW_LINE == ch) && CARRIAGE_RETURN != ch {
                v.push(ind);
            }
            v
        })
}

pub fn get_ngram_counts(idxs: &Vec<usize>, mmap: &Mmap) -> Vec<(String, usize)> {
    let mut ngram_counts = Vec::from_iter(
        idxs.iter()
            .enumerate()
            .filter(|(n, _)| n + NGRAM_LEN < idxs.len())
            .fold(HashMap::new(), |mut h, (n, _)| {
                let ngram = std::str::from_utf8(&mmap[idxs[n] + 1..idxs[n + NGRAM_LEN]])
                    .unwrap()
                    .trim()
                    .chars()
                    .filter(|ch| ch.is_alphabetic() || ch.is_whitespace())
                    .map(|ch| ch.to_ascii_uppercase() as char)
                    .collect::<String>();
                if NGRAM_LEN == ngram.chars().filter(|ch| ch.is_whitespace()).count() + 1 {
                    *h.entry(ngram).or_insert(0) += 1;
                }
                h
            }),
    );
    ngram_counts.sort_unstable_by(|(s1, _), (s2, _)| s1.cmp(s2));
    ngram_counts.sort_by(|&(_, c1), &(_, c2)| c2.cmp(&c1));
    ngram_counts
}

pub fn get_top_counts(ngram_counts: &Vec<(String, usize)>) -> String {
    ngram_counts
        .into_iter()
        .take(TOP_LIMIT)
        .fold(String::new(), |mut s, (ngram, count)| {
            s.push_str(format!("{:?} {}\n", ngram, count).as_str());
            s
        })
}

pub fn from_path(path: &str) -> Result<String, std::io::Error> {
    let file = File::open(path).unwrap();
    let mmap = unsafe { MmapOptions::new().map(&file)? };
    let idxs = load_word_indexes(&mmap);
    println!("Detected {} words in file {}", &idxs.len(), path);
    Ok(get_top_counts(&get_ngram_counts(&idxs, &mmap)))
}

pub fn from_stdin() -> Result<String, std::io::Error> {
    //let stdin = std::io::stdin();
    //let lines = stdin.lock();
    Ok(String::new())
    //let nbuf = load_words(&mut lines);
    //let ngram_counts = get_ngram_counts(&nbuf);
    //get_top_counts(&ngram_counts)
}
