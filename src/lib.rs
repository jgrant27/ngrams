use std::collections::HashMap;
use std::fs::File;
use std::iter::FromIterator;

use memmap::{Mmap, MmapOptions};

const NGRAM_LEN: usize = 3;
const TOP_LIMIT: usize = 100;
const SPACE: u8 = ' ' as u8;

pub fn load_word_indexes(mmap: &Mmap) -> Vec<usize> {
    let mut idxs: Vec<usize> = Vec::new();
    let mut last_idx = 0;
    for ind in 0..mmap.len() {
        if SPACE == mmap[ind] || '\n' as u8 == mmap[ind] {
            if 0 == ind || ind - 1 != last_idx {
                idxs.push(ind);
                last_idx = ind;
            }
        }
    }
    idxs
}

pub fn get_ngram_counts(idxs: &Vec<usize>, mmap: &Mmap) -> Vec<(String, usize)> {
    let mut cmap: HashMap<String, usize> = HashMap::new();
    for n in 0..idxs.len() {
        if n + NGRAM_LEN < idxs.len() {
            let start = idxs[n] + 1;
            let end = idxs[n + NGRAM_LEN];
            let ngram = std::str::from_utf8(&mmap[start..end])
                .unwrap()
                .chars()
                .filter(|ch| ch.is_alphabetic() || ' ' == *ch)
                .collect::<String>()
                .to_uppercase()
                .trim()
                .to_string();
            let cnt = ngram.chars().filter(|ch| ' ' == *ch).count() + 1;
            if NGRAM_LEN == cnt {
                *cmap.entry(ngram).or_insert(0) += 1;
            }
        }
    }
    let mut ngram_counts = Vec::from_iter(cmap);
    ngram_counts.sort_unstable_by(|(s1, _), (s2, _)| s1.cmp(s2));
    ngram_counts.sort_by(|&(_, c1), &(_, c2)| c2.cmp(&c1));
    ngram_counts
}

pub fn get_top_counts(ngram_counts: &Vec<(String, usize)>) -> String {
    let mut res: String = String::new();
    for (ngram, count) in ngram_counts.into_iter().take(TOP_LIMIT) {
        res.push_str(format!("{:?} {}\n", ngram, count).as_str());
    }
    res
}

pub fn from_path(path: &str) -> Result<String, std::io::Error> {
    let file = File::open(path).unwrap();
    let mmap = unsafe { MmapOptions::new().map(&file)? };
    let idxs = load_word_indexes(&mmap);
    println!("Detected {} words in file {}", &idxs.len(), path);
    let ngram_counts = get_ngram_counts(&idxs, &mmap);
    Ok(get_top_counts(&ngram_counts))
}

pub fn from_stdin() -> Result<String, std::io::Error> {
    let stdin = std::io::stdin();
    let lines = stdin.lock();
    Ok(String::new())
    //let nbuf = load_words(&mut lines);
    //let ngram_counts = get_ngram_counts(&nbuf);
    //get_top_counts(&ngram_counts)
}
