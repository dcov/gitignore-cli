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

fn main() {
    let files_dir = PathBuf::from(env::var(ENV_HOME)
        .expect(format!("{} is not set.", ENV_HOME).as_str()));
        
    let matches = App::new("gitignore")
        .version("0.2.1")
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
            .help("Remove the specified file_stems from .gitignore file instead of adding them"))
        .arg(Arg::with_name("file_stems")
            .multiple(true)
            .required(true)
            .help("The case-insensitive file stems to search for, e.g. 'rust' will match 'rust.gitignore', 'RUST.gitignore', etc."))
        .get_matches();

    if let Some(file_stems) = matches.values_of("file_stems") {
        let current_dir_path = env::current_dir().expect("Could not determine current directory");

        let write_path = write_path::lookup(&current_dir_path, !matches.is_present("current_dir"));
        println!("Writing to {}", write_path.to_str().unwrap());

        if matches.is_present("remove") {
            generator::remove(&write_path, &file_stems.collect());
        } else {
            let read_paths = read_paths::lookup(&files_dir, &file_stems.collect());
            for path in &read_paths {
                println!("Reading from {}", path.to_str().unwrap());
            }

            generator::insert(&write_path, &read_paths);
        }

        println!("Completed successfully!");
    }
}

