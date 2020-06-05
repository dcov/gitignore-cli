//! Read paths lookup.
//!
//! This module contains the lookup functionality for the '*.gitignore' file paths from
//! which the file contents will be read and used to generate the resulting '.gitignore' file.
//!
//! Note: It returns the paths insteaad of the files themselves because the paths will
//! be needed when generating the contents of the write file.

use std::fs::{self, DirEntry, File};
use std::io;
use std::path::PathBuf;

fn path_is_gitignore_file(path: &PathBuf) -> bool {
    match path.extension() {
        Some(extension) => {
            match extension.to_str() {
                Some(extension_str) => {
                    extension_str == "gitignore"
                },
                None => {
                    println!("Error converting OsStr");
                    false
                }
            }
        },
        None => false
    }
}

fn validate_entry_result(entry_result: Result<DirEntry, io::Error>) -> Result<(String, File), ()> {
    let entry_path = match entry_result {
        Ok(entry) => entry.path(),
        Err(_) => {
            return Err(());
        }
    };

    if entry_path.is_dir() || !path_is_gitignore_file(&entry_path) {
        return Err(());
    }

    let stem_str = match entry_path.file_stem() {
        Some(stem) => {
            match stem.to_str() {
                Some(stem_str) => stem_str,
                None => {
                    println!("Error converting OsStr");
                    return Err(());
                }
            }
        },
        None => {
            return Err(());
        }
    };

    let file = match File::open(entry_path.clone()) {
        Ok(file) => file,
        Err(_) => {
            println!("Error opening file: {}", entry_path.to_str().unwrap());
            return Err(())
        }
    };
    
    Ok((String::from(stem_str), file))
}

fn index_of(value: &str, from: &Vec<&str>) -> Option<usize> {
    let lowercase_value = of.to_ascii_lowercase();
    for (index, entry) in from.iter().enumerate() {
        let lowercase_entry = entry.to_ascii_lowercase();
        if lowercase_value == lowercase_entry {
            return Some(index);
        }
    }

    None
}

pub fn find() {
}

