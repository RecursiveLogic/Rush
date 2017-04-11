use std::env;
use std::fs;
use std::io;
use std::process::Command;
use std::path::Path;
use std::ffi::OsString;

use nix::unistd::{fork, ForkResult};

fn exec_process(bin_path: &str, argument: &str) {
    let mut builder = Command::new(bin_path);

    if !argument.is_empty() {
        builder.arg(argument);
    }

    if let Ok(mut builder) = builder.spawn() {
        match fork() {
            Ok(ForkResult::Parent { child, .. }) => {
                builder
                    .expect("Command wasn't running");
                println!("New child PID: {}", child);
            },
            Ok(ForkResult::Child) => println!("I'm a new child process"),
            Err(e) => println!("Fork failed: {}", e)
        }
    } else {
        println!("Failed to execute process");
    }
}

fn visit_dir(dir: &Path, cmd: &str, argument: &str) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let empty_dir = OsString::new();
            let entry = entry?;
            let path = entry.path();
            let bin = path.iter().last().unwrap_or(&empty_dir);

            if bin == cmd {
                exec_process(path.to_str().unwrap(), argument);
                return Ok(());
            }
        }
    }
    Ok(())
}

pub fn find_path_cmd(cmd: &str, argument: &str) {
    let key = "PATH";

    match env::var_os(key) {
        Some(paths) => {
            let mut directories = env::split_paths(&paths).collect::<Vec<_>>();
                directories.sort();
                directories.dedup();
            for path in directories.iter() {
                visit_dir(&path, &cmd, &argument);
            }
        },
        None => println!("{} is not defined in the environment", key)
    }
}
