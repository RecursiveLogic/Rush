use std::env;
use std::fs::{self, File};
use std::io::{self, Write};
use std::process::{Command, Stdio};
use std::path::Path;
use std::ffi::OsString;

use nix::unistd::{fork, ForkResult};

fn exec_cmd(bin_path: &str, argument: &str, input: &str) {
    let mut builder = Command::new(bin_path);

    if !argument.is_empty() {
        builder.arg(argument);
    }

    if !input.is_empty() {
        let mut child = builder
            .output()
            .unwrap_or_else(|e| {
                panic!("Failed to execute process: {}", e);
            });

        if child.status.success() {
            let s = String::from_utf8_lossy(&child.stdout);
            // let mut f = try!(File::create(&Path::new(&input)));
            let mut f = File::create(&Path::new(&input)).unwrap();
            f.write_all(s.as_bytes());
        }
    } else {
        let mut child = builder
            .stdout(Stdio::inherit())
            .spawn()
            .expect("Command didn't execute successfully");
    }
}

fn visit_dir(dir: &Path, cmd: &str, argument: &str, input: &str) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let empty_dir = OsString::new();
            let entry = entry?;
            let path = entry.path();
            let bin = path.iter().last().unwrap_or(&empty_dir);

            if bin == cmd {
                exec_cmd(path.to_str().unwrap(), argument, input);
                return Ok(());
            }
        }
    }
    Ok(())
}

pub fn find_path_cmd(cmd: &str, argument: &str, input: &str) {
    let key = "PATH";

    match env::var_os(key) {
        Some(paths) => {
            let mut directories = env::split_paths(&paths).collect::<Vec<_>>();
                directories.sort();
                directories.dedup();
            for path in directories.iter() {
                visit_dir(&path, &cmd, &argument, &input);
            }
        },
        None => println!("{} is not defined in the environment", key)
    }
}
