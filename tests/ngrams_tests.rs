#[cfg(test)]
mod ngram_tests {

    use std::fs;
    use ngrams::{clean_word, from_path};

    #[test]
    fn clean_word_test() {
        assert_eq!("NGRAM", clean_word("! ngram. "));
        assert_eq!("NGRAM", clean_word("n-gram !!!"));
    }

    #[test]
    fn from_path_test() {
        assert_eq!(from_path("./text/pg2009.txt").trim(),
                   fs::read_to_string("./text/pg2009_res.txt").unwrap().trim());
        assert_eq!(from_path("./text/1661.txt").trim(),
                   fs::read_to_string("./text/1661_res.txt").unwrap().trim());
    }
}
