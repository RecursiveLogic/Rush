use std::path::{self, Path, PathBuf};
use std::env;
use std::fs::PathExt;

pub fn change_dir(input: &str) {
    let input = input
        .split_whitespace()
        .collect::<Vec<&str>>();

    if input.is_empty() {
        env::set_current_dir(Path::new(env::var("HOME")))
            .expect("No HOME variable set")
            .as_str();
    } else {
        let mut p_buffer = PathBuf::new();
    }
}
