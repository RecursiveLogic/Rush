#![allow(unused_must_use)]
#![allow(unused_imports)]
extern crate nix;

mod commands;
mod utils;

use std::{thread, time};
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

#[allow(dead_code)]
fn grab_arguments() {
    let mut arguments = vec![];
    for argument in env::args() {
        arguments.push(argument);
    }
    arguments.sort()
}

fn main() {
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    let mut stdin = stdin.lock();
    let mut stdout = stdout.lock();
    let mut buffer = String::new();
    let empty_dir = OsString::new();

    println!("** rush **\n");

    loop {
        let root_dir = env::home_dir().unwrap();
        let curr_dir = env::current_dir().unwrap();
        let last_dir = curr_dir.iter().last().unwrap_or(&empty_dir);

        let output = if curr_dir == root_dir {
            ["rush:~ ", "λ "].join("")
        } else {
            ["rush:", last_dir.to_str().unwrap(), " λ "].join("")
        };

        write!(stdout, "{}", output);
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
        let argument = commands.get(1).cloned().unwrap_or("");
        let argument2 = commands.get(2).cloned().unwrap_or("");
        let input = commands.get(3).cloned().unwrap_or("");

        if argument == ">" {
            utils::bin_exec::find_path_cmd(command, "", argument2);
        } else if argument2 == ">" {
            utils::bin_exec::find_path_cmd(command, argument, input);
        } else {
            match command {
                "pwd" => println!("{}", curr_dir.display()),
                "cd" => commands::change_dir::change_dir(argument),
                "touch" => touch(&Path::new(argument))
                    .unwrap_or_else(|why| {
                    println!("! {:?}", why.kind());
                }),
                "rm" => fs::remove_file(argument)
                    .unwrap_or_else(|why| {
                    println!("! {:?}", why.kind());
                }),
                "mkdir" => fs::create_dir(argument)
                    .unwrap_or_else(|why| {
                    println!("! {:?}", why.kind())
                }),
                "rmdir" => fs::remove_dir(argument)
                    .unwrap_or_else(|why| {
                    println!("! {:?}", why.kind());
                }),
                "sleep" => {
                    let input_time = argument.parse::<u64>().unwrap();
                    let sleep_time = time::Duration::from_millis(input_time * 1000);
                    let now = time::Instant::now();

                    thread::sleep(sleep_time);
                    println!("Sleep {:?}", now.elapsed());
                },
                "exit" => break,
                "help" => println!("Sorry, you're on your own for now"),
                _ => utils::bin_exec::find_path_cmd(command, argument, "")
            }
        }
    }
}
