use std::env;
use std::fs;
use clap::{Arg, App};

mod write_file;
mod generator;

static ENV_HOME: &str = "GITIGNORE_HOME";

fn main() {
    let files_dir = match env::var(ENV_HOME) {
        Ok(value) => value,
        _ => {
            println!("{} is not set.", ENV_HOME);
            return;
        }
    };

    let dir = match fs::read_dir(files_dir) {
        Ok(d) => d,
        _ => {
            println!("{} is not a valid directory.", ENV_HOME);
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

    if let Some(language_names) = matches.values_of("language_names") {
        let current_dir_path = match env::current_dir() {
            Ok(p) => p,
            _ => {
                println!("Could not determine current dir path.");
                return;
            }
        };

        let write_file = match write_file::open(current_dir_path, true) {
            Ok(f) => f,
            _ => {
                println!("Not in a git repository.");
                return;
            }
        };

        generator::generate(dir, language_names.collect(), write_file);
    }
}

