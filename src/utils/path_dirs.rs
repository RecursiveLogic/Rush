use std::env;
use std::fs;
use std::io;
use std::path::Path;

fn visit_dirs(dir: &Path) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            println!("Visiting dir: {:?}", entry);
            // if path.is_dir() {
            //     visit_dirs(&path)?;
            // }
        }
    }
    Ok(())
}

// Need to add callback functionality to execute bin commands
pub fn get_path_dirs() {
    let key = "PATH";

    match env::var_os(key) {
        Some(paths) => {
            for path in env::split_paths(&paths) {
                visit_dirs(&path);
            }
        },
        None => println!("{} is not defined in the environment", key)
    }
}
