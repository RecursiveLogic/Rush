use std::io;

use regex::Regex;

struct Parser;

impl Parser {
    fn grab_tokens(&self, input: &str) -> Vec<&str> {
        let tokens = Regex::new(r"-?[a-zA-Z]+|[.<>|&$]+").unwrap();
        let matches: Vec<&str> = tokens
            .find_iter(input)
            .map(|x| x.as_str())
            .collect();

        matches
    }
}
