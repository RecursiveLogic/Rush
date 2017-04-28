#![allow(unused_must_use)]
#![allow(unused_imports)]
extern crate nix;
extern crate regex;

use std::{thread, time};
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufReader, Write};
use std::io::prelude::*;
use std::os::unix;
use std::path::{self, Path};
use std::env;
use std::process::{Command, Stdio};
use std::ffi::OsString;

use nix::unistd::{fork, ForkResult};

struct Shell;

impl Shell {
    fn change_dir(&self, input: &str) {
        let root = env::home_dir().unwrap();
        let c_dir = env::current_dir().unwrap();
        let mut prev_dirs = c_dir.clone();
                prev_dirs.pop();
        let prev_dir = prev_dirs.join("");
        let path = Path::new(input);

        if input.is_empty() {
            env::set_current_dir(root)
                .expect("No home directory found")
        } else if !path.is_dir() {
            println!("-rush: cd: {}: No such directory", input);
        } else {
            match input {
                "~" | "~/" => env::set_current_dir(&root).unwrap(),
                "." => env::set_current_dir(&c_dir).unwrap(),
                ".." => env::set_current_dir(&prev_dir).unwrap(),
                _ => env::set_current_dir(path).unwrap()
            }
        }
    }
    fn save_history(&self, cmd: &str) -> io::Result<()> {
        let root = env::home_dir().unwrap();
        let bash_history = [root.to_str().unwrap(), "/.bash_history"].join("");
        let mut f = OpenOptions::new().append(true).open(bash_history)?;

        f.write_all([cmd, "\n"].join("").as_bytes())?;
        Ok(())
    }
    fn get_history(&self) {
        let root = env::home_dir().unwrap();
        let bash_history = [root.to_str().unwrap(), "/.bash_history"].join("");
        let mut f = File::open(bash_history).unwrap();
        let mut content = String::new();

        f.read_to_string(&mut content);

        print!("{}", content);
    }
}

fn exec_cmd(bin_path: &str, argument: &str, input: &str) {
    let mut builder = Command::new(bin_path);

    if !argument.is_empty() {
        builder.arg(argument);
    }

    if !input.is_empty() {
        let child = builder
            .output()
            .unwrap_or_else(|e| {
                panic!("Failed to execute process: {}", e);
            });

        if child.status.success() {
            let s = String::from_utf8_lossy(&child.stdout);
            let mut f = File::create(&Path::new(&input)).unwrap();
            f.write_all(s.as_bytes());
        }
    } else {
        let mut child = builder
            .stdout(Stdio::inherit())
            .spawn()
            .expect("Command didn't execute successfully");

        let ecode = child.wait().unwrap_or_else(|e| {
            panic!("Failed to wait on child: {}", e);
        });

        assert!(ecode.success());
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

fn find_path_cmd(cmd: &str, argument: &str, input: &str) {
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

fn touch(path: &Path) -> io::Result<()> {
    match OpenOptions::new()
        .create(true)
        .write(true)
        .open(path) {
        Ok(_) => Ok(()),
        Err(e) => Err(e)
    }
}

#[test]
fn return_home_if_empty_arg() {
    let path = Path::new("/usr/");
    let prev_dir = env::set_current_dir(path);
    Shell.change_dir("");
    let root = env::home_dir().unwrap();
    let c_dir = env::current_dir().unwrap();
    assert_eq!(root, c_dir);
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
            "rush:~ λ ".to_string()
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

        Shell.save_history(&command);

        if argument == ">" {
            find_path_cmd(command, "", argument2);
        } else if argument2 == ">" {
            find_path_cmd(command, argument, input);
        } else {
            match command {
                "pwd" => println!("{}", curr_dir.display()),
                "cd" => Shell.change_dir(argument),
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
                "history" => Shell.get_history(),
                "sleep" => {
                    let input_time = argument.parse::<u64>().unwrap();
                    let sleep_time = time::Duration::from_millis(input_time * 1000);
                    let now = time::Instant::now();

                    thread::sleep(sleep_time);
                    println!("Sleep {:?}", now.elapsed());
                },
                "help" => println!("Sorry, you're on your own for now"),
                "exit" => break,
                _ => find_path_cmd(command, argument, "")
            }
        }
    }
}
