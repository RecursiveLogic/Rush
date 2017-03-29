use std::env;
use std::fs;
use std::io;
use std::process::Command;
use std::path::Path;
use std::ffi::OsString;

fn bin_command(bin_path: &str, argmt: &str) {
    let mut child = Command::new(bin_path)
        .arg(argmt)
        .spawn()
        .expect("Failed to execute child");

    let ecode = child
        .wait()
        .expect("Failed to wait on child");
}

fn visit_dir(dir: &Path, cmd: &str, argmt: &str) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let empty_dir = OsString::new();
            let entry = entry?;
            let path = entry.path();
            let bin = path.iter().last().unwrap_or(&empty_dir);
            if bin == cmd {
                bin_command(path.to_str().unwrap(), argmt);
                return Ok(());
            }
        }
    }
    Ok(())
}

pub fn get_path_dirs(cmd: &str, argmt: &str) {
    let key = "PATH";
    match env::var_os(key) {
        Some(paths) => {
            for path in env::split_paths(&paths) {
                visit_dir(&path, &cmd, &argmt);
            }
        },
        None => println!("{} is not defined in the environment", key)
    }
}
