#![allow(unused_must_use)]
#![allow(unused_imports)]
use std::path::{self, Path, PathBuf};
use std::env;
// use std::fs::PathExt;

pub fn change_dir(input: &str) {
    let root = Path::new(env::var("HOME"));
    let input = input
        .split_whitespace()
        .collect::<Vec<&str>>();

    if input.is_empty() {
        env::set_current_dir(root)
            .expect("No HOME variable set")
            .as_str();
    } else {
        let buffer = PathBuf::new();
        assert!(env::set_current_dir(buffer).is_ok());
        println!("Changed working directory to {}!", buffer.display());
    }
}
