use std::io;

use regex::Regex;

fn grab_tokens(input: &str) {
    match input {
        ">" => println!("Overwrite Redirect IO"),
        ">>" => println!("Append Redirect IO"),
        ">&" => println!("Overwrite Redirect IO || STDERR"),
        ">>&" => println!("Append Redirect IO || STDERR"),
        "|" => println!("Pipe STDOUT"),
        "|&" => println!("Pipe STDOUT || STDERR"),
        "&" => println!("Fork process"),
        "&&" => println!("And"),
        "||" => println!("Or"),
        ";" => println!("Terminate"),
        "$$" => println!("Print PID of executed process"),
        _ => println!("No match")
    }
}

fn parse_line(input: &str) {
    let commands = input.split("|");
    let command = commands[0];
    let arguments = command.split_whitespace();
}
