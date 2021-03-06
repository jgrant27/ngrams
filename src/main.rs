use ngrams::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let output = if 1 == args.len() {
        format!("STDIN:\n{}\n", from_stdin())
    } else {
        let mut res: String = String::new();
        for path in &args[1..] {
            let contents = from_path(path.as_str());
            res.push_str(format!("{}:\n{}\n", path, contents).as_str());
        }
        res
    };
    println!("{}", output);
}
