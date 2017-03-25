#![allow(unused_must_use)]
#![allow(unused_imports)]
// mod commands;

use std::fs::{self, File, OpenOptions};
use std::io;
use std::io::prelude::*;
use std::os::unix;
use std::path::Path;
use std::env;
use std::process::Command;
use std::ffi::OsString;

fn touch(path: &Path) -> io::Result<()> {
    match OpenOptions::new()
        .create(true)
        .write(true)
        .open(path) {
        Ok(_) => Ok(()),
        Err(e) => Err(e)
    }
}

fn bin_command(s: &str, argmnt: &str) {
    let mut child = Command::new(["/bin/", s].join(""))
        .arg(argmnt)
        .spawn()
        .expect("Failed to execute child");

    let ecode = child
        .wait()
        .expect("Failed to wait on child");

    assert!(ecode.success());
}

fn main() {
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    let mut stdin = stdin.lock();
    let mut stdout = stdout.lock();
    let mut buffer = String::new();
    let empty_dir = OsString::new();
    // let root = Path::new(env::var("HOME"));

    println!("** Rsh **\n");

    loop {
        let curr_dir = env::current_dir().unwrap();
        let last_dir = curr_dir.iter().last().unwrap_or(&empty_dir);

        write!(stdout, "{}", [":", last_dir.to_str().unwrap(), " λ "].join(""));

        stdout.flush();
        buffer.clear();

        stdin
            .read_line(&mut buffer)
            .expect("Failed to parse command");

        let commands = buffer
            .trim()
            .split_whitespace()
            .collect::<Vec<_>>();

        let command = &commands[0] as &str;

        match command {
            "pwd" => println!("{}", curr_dir.display()),
            "touch" => touch(&Path::new(&commands[1] as &str))
                .unwrap_or_else(|why| {
                println!("! {:?}", why.kind());
            }),
            "rm" => fs::remove_file(&commands[1] as &str)
                .unwrap_or_else(|why| {
                println!("! {:?}", why.kind());
            }),
            "mkdir" => fs::create_dir(&commands[1] as &str)
                .unwrap_or_else(|why| {
                println!("! {:?}", why.kind())
            }),
            "rmdir" => fs::remove_dir(&commands[1] as &str)
                .unwrap_or_else(|why| {
                println!("! {:?}", why.kind());
            }),
            "help" => println!("Sorry, you're on your own for now"),
            _ => bin_command(command, &commands[1] as &str)
            // _ => println!("Rsh: {} <- command not found", command)
        }
    }
}
