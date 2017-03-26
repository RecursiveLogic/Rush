#![allow(unused_must_use)]
#![allow(unused_imports)]
use std::path::{self, Path};
use std::env;

pub fn change_dir(input: &str) {
    let root = env::home_dir().unwrap();
    let c_dir = env::current_dir().unwrap();
    let mut prev_dirs = c_dir.clone();
            prev_dirs.pop();
    let prev_dir = prev_dirs.join("");
    let buffer = Path::new(input);

    if input.is_empty() {
        env::set_current_dir(root)
            .expect("No home directory found")
    } else {
        match input {
            "." => env::set_current_dir(&c_dir).unwrap(),
            ".." => env::set_current_dir(prev_dir).unwrap(),
            _ => env::set_current_dir(buffer).unwrap()
        }
    }
}
