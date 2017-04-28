use std::io;

use regex::Regex;

fn grab_tokens(input: &str) -> io::Result<()> {
    let tokens = Regex::new(r"[|&<>;$]+").unwrap();
    let matches: Vec<&str> = tokens
        .find_iter(input)
        .map(|x| x.as_str())
        .collect();

    Ok(matches)
}

fn parse_line(input: &str) {
    let commands = input.split("|");
    let command = commands[0];
    let arguments = command.split_whitespace();
}
