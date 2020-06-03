use std::io;
use std::fs::{ReadDir, DirEntry, File};
use std::path::{Path, PathBuf};

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

fn find_index(from: &Vec<&str>, of: &str) -> Option<usize> {
    let lowercase_value = of.to_ascii_lowercase();
    for (index, entry) in from.iter().enumerate() {
        let lowercase_entry = entry.to_ascii_lowercase();
        if lowercase_value == lowercase_entry {
            return Some(index);
        }
    }

    None
}

pub fn generate(from: ReadDir, mut with: Vec<&str>, into: File) {
    for entry_result in from {
        if let Ok((name, file)) = validate_entry_result(entry_result) {
            if let Some(remove_index) = find_index(&with, name.as_str()) {
                with.remove(remove_index);
            }
        }
    }
}

