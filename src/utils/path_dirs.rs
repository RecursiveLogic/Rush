use std::env;
use std::fs;
use std::io;
use std::process::Command;
use std::path::Path;
use std::ffi::OsString;

fn bin_command(bin_path: &str, argument: &str) {
    let mut child = Command::new(bin_path)
        .arg(argument)
        .spawn()
        .expect("Failed to execute child");

    // let ecode = child
    //     .wait()
    //     .expect("Failed to wait on child");
    // assert!(ecode.success())
}

fn visit_dir(dir: &Path, cmd: &str, argument: &str) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let empty_dir = OsString::new();
            let entry = entry?;
            let path = entry.path();
            let bin = path.iter().last().unwrap_or(&empty_dir);
            if bin == cmd {
                bin_command(path.to_str().unwrap(), argument);
                return Ok(());
            }
        }
    }
    Ok(())
}

pub fn get_path_dirs(cmd: &str, argument: &str) {
    let key = "PATH";
    match env::var_os(key) {
        Some(paths) => {
            for path in env::split_paths(&paths) {
                visit_dir(&path, &cmd, &argument);
            }
        },
        None => println!("{} is not defined in the environment", key)
    }
}