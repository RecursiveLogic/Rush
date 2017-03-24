use std::fs::{self, File, OpenOptions};
use std::io;
use std::io::prelude::*;
use std::os::unix;
use std::path::Path;
use std::env;
use std::process::Command;

fn touch(path: &Path) -> io::Result<()> {
    match OpenOptions::new()
        .create(true)
        .write(true)
        .open(path) {
        Ok(_) => Ok(()),
        Err(e) => Err(e)
    }
}

fn echo(s: &str, path: &Path) -> io::Result<()> {
    let mut f = try!(File::create(path));
    f.write_all(s.as_bytes())
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

fn pwd() {
    for (key, value) in env::vars() {
        if key == "PWD" {
            println!("{}", value);
        }
    }
}

fn main() {
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    let mut stdin = stdin.lock();
    let mut stdout = stdout.lock();
    let mut buffer = String::new();

    println!("** Rsh **\n");

    loop {
        write!(stdout, "λ ");
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
            "pwd" => pwd(),
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
            "echo" => echo(&commands[1] as &str, &Path::new(&commands[2] as &str))
                .unwrap_or_else(|why| {
                println!("! {:?}", why.kind());
            }),
            "help" => println!("Sorry, you're on your own for now"),
            _ => bin_command(command, &commands[1] as &str)
            // _ => println!("Rsh: {} <- command not found", command)
        }
    }
}
