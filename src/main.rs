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
    let files_dir = match env::var(ENV_HOME) {
        Ok(value) => PathBuf::from(value),
        _ => {
            println!("{} is not set.", ENV_HOME);
            return;
        }
    };
        
    let matches = App::new("gitignore")
        .version("0.1.0")
        .author("Diego Covarrubias <dcov@pm.me>")
        .about("A tool to manage .gitignore files")
        .arg(Arg::with_name("files")
            .multiple(true)
            .required(true))
        .get_matches();

    if let Some(_language_names) = matches.values_of("language_names") {
        let current_dir_path = match env::current_dir() {
            Ok(p) => p,
            _ => {
                println!("Could not determine current dir path.");
                return;
            }
        };

        let _write_path = write_path::lookup(&current_dir_path, false);

        let _read_paths = read_paths::lookup(&files_dir, &vec![""]);

        generator::generate(&_write_path, &_read_paths);
    }
}

