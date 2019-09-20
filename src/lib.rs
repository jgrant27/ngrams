use std::collections::{HashMap};
use std::fs::File;
use std::iter::FromIterator;

use memmap::{MmapOptions, Mmap};

const NGRAM_LEN: usize = 3;
const TOP_LIMIT: usize = 100;
const SPACE: u8 = ' ' as u8;

pub fn load_word_indexes(mmap: &Mmap) -> Vec<usize> {
    let mut idxs: Vec<usize> = Vec::new();
    for ind in 0..mmap.len() {
        if SPACE == mmap[ind] || '\n' as u8 == mmap[ind] {
            idxs.push(ind);
        }
    }
    idxs
}

pub fn get_ngram_counts<'a>(idxs: &Vec<usize>, mmap: &'a Mmap) -> Vec<(String, usize)> {
    let mut cmap: HashMap<String, usize> = HashMap::new();
    for (n, _) in idxs.iter().enumerate() {
        let start: usize = idxs[n] + 1;
        if n + NGRAM_LEN < idxs.len() - 1 {
            let end: usize = idxs[n + NGRAM_LEN];
            let ngram = std::str::from_utf8(&mmap[start..end]).unwrap()
                .chars().filter(|ch| ch.is_alphabetic() || ' ' == *ch)
                .collect::<String>().to_uppercase().trim().to_string();
            if !ngram.is_empty() && NGRAM_LEN == ngram.split(' ').collect::<Vec<_>>().len() {
                *cmap.entry(ngram).or_insert(0) += 1;
            }
        }
    }
    let mut ngram_counts = Vec::from_iter(cmap);
    ngram_counts.sort_unstable_by(|(s1, _), (s2, _)| s1.cmp(s2));
    ngram_counts.sort_by(|&(_, c1), &(_, c2)| c2.cmp(&c1));
    ngram_counts
}

pub fn get_top_counts<'a>(ngram_counts: &'a Vec<(String, usize)>) -> String {
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
