#[cfg(test)]
mod ngram_tests {

    use ngrams::from_path;
    use std::fs;

    #[test]
    fn from_path_test() {
        assert_eq!(
           from_path("./text/1661.txt").unwrap().trim(),
           fs::read_to_string("./text/1661_res.txt").unwrap().trim()
        );
        assert_eq!(
            from_path("./text/pg2009.txt").unwrap().trim(),
            fs::read_to_string("./text/pg2009_res.txt").unwrap().trim()
        );
    }
}
