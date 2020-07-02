//! gitignore-cli

#[cfg(test)]
#[macro_use]
extern crate cascade;

mod generator;
mod read_paths;
mod write_path;

use std::env;
use std::path::PathBuf;
use clap::{Arg, App};

static ENV_HOME: &str = "GITIGNORE_HOME";

fn print_block_names(names: &Vec<String>) {
    for name in names {
        println!("{}", name);
    }
}

fn sync(write_path: &PathBuf, files_dir: &PathBuf, print_names: bool) {
    let names = generator::get_block_names(&write_path);
    let read_paths = read_paths::lookup(&files_dir, &names);
    generator::insert(&write_path, &read_paths);
    println!("Synced successfully!");

    if print_names {
        print_block_names(&names);
    }
}

fn main() {
    let files_dir = PathBuf::from(env::var(ENV_HOME)
        .expect(format!("{} is not set.", ENV_HOME).as_str()));
        
    let matches = App::new("gitignore")
        .version("0.4.0")
        .about("Manage .gitignore files")
        .arg(Arg::with_name("current_dir")
            .short("c")
            .takes_value(false)
            .required(false)
            .help("Generate the .gitignore in the current dir.")
            .long_help("Generate the .gitignore in the current dir instead of searching for the git repo's root directory."))
        .arg(Arg::with_name("remove")
            .short("r")
            .takes_value(false)
            .required(false)
            .help("Remove the specified file_stems from .gitignore file instead of adding them."))
        .arg(Arg::with_name("list")
            .short("l")
            .takes_value(false)
            .required(false)
            .help("List the current file_stems.")
            .long_help("List the current file_stems. This will run after any other commands."))
        .arg(Arg::with_name("sync")
            .short("s")
            .takes_value(false)
            .required(false)
            .help("Sync the exisiting .gitignore blocks with the source $GITIGNORE_HOME files."))
        .arg(Arg::with_name("file_stems")
            .multiple(true)
            .required_unless_one(&["list", "sync"])
            .help("The case-insensitive file stems to search for, e.g. 'rust' will match 'rust.gitignore', 'RUST.gitignore', etc."))
        .get_matches();

    let current_dir_path = env::current_dir().expect("Could not determine current directory");
    let write_path = write_path::lookup(&current_dir_path, !matches.is_present("current_dir"));

    if let Some(file_stems) = matches.values_of("file_stems") {
        println!("Writing to {}", write_path.to_str().unwrap());

        if matches.is_present("remove") {
            generator::remove(&write_path, &file_stems.collect());
        } else {
            let read_paths = read_paths::lookup(&files_dir, &file_stems.map(String::from).collect());
            for path in &read_paths {
                println!("Reading from {}", path.to_str().unwrap());
            }

            generator::insert(&write_path, &read_paths);
        }

        println!("Generated successfully!");
    }

    if matches.is_present("sync") {
        sync(&write_path, &files_dir, matches.is_present("list"));
    } else if matches.is_present("list") {
        print_block_names(&generator::get_block_names(&write_path));
    }
}

#[cfg(test)]
mod tests {
    
    use super::*;
    use std::fs;
    use cascade;
    use tempfile;

    #[test]
    fn test_sync() {
        let dir = tempfile::tempdir().unwrap();
        let dir_path = dir.path();
        let write_path = dir_path.join("write_file");

        let rust_path = dir_path.join("rust.gitignore");
        let mut rust_contents = "target/";
        fs::write(rust_path.clone(), rust_contents).unwrap();

        let js_path = dir_path.join("js.gitignore");
        let mut js_contents = "dist/";
        fs::write(js_path.clone(), js_contents).unwrap();

        generator::insert(&write_path, &vec![rust_path.clone(), js_path.clone()]);

        dir.close().unwrap();
    }
}

