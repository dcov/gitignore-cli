//! Write path lookup.
//!
//! This module contains the lookup functionality for the '.gitignore' file that
//! will be written to.
//!
//! Note: It returns the path to the file only, it does not open any files.

use std::fs::{self, DirEntry};
use std::path::PathBuf;

fn search_dir_for_gitignore_path(dir_path: &PathBuf, only_git_root: bool) -> Option<PathBuf> {
    let dir = fs::read_dir(dir_path).unwrap();
    let mut is_git_root = false;
    let mut entry: Option<DirEntry> = None;
    for entry_result in dir {
        if let Ok(dir_entry) = entry_result {
            if let Some(file_name) = dir_entry.file_name().to_str() {
                match file_name {
                    ".gitignore" => {
                        if only_git_root && !is_git_root {
                            entry = Some(dir_entry);
                            continue;
                        } else {
                            return Some(dir_entry.path());
                        }
                    },
                    ".git" => {
                        if entry.is_some() {
                            return Some(entry.unwrap().path());
                        }
                        is_git_root = true;
                        continue;
                    },
                    _ => continue
                }
            }
        }
    }

    if entry.is_none() && (!only_git_root || is_git_root) {
        return Some(dir_path.join(PathBuf::from(".gitignore")));
    }

    None
}

pub fn lookup(from_dir_path: &PathBuf, only_git_root: bool) -> PathBuf {
    let mut current_dir_path = from_dir_path.clone();
    loop {
        if let Some(path) = search_dir_for_gitignore_path(&current_dir_path, only_git_root) {
            return path;
        }

        if !only_git_root || !current_dir_path.pop() {
            break;
        }
    }

    panic!("Could not find .gitignore write file.");
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::panic::catch_unwind;
    use tempfile;

    #[test]
    fn test_lookup() {
        let root_dir = tempfile::tempdir().unwrap();
        let root_dir_path = root_dir.path().to_path_buf();

        // Add some random files into the directory
        fs::File::create(root_dir_path.join("random.txt")).unwrap();
        fs::File::create(root_dir_path.join("fizz.h")).unwrap();
        fs::File::create(root_dir_path.join("buzz.c")).unwrap();

        // Assert that searching for a .gitignore file path when [only_git_root] is [false]
        // will return a path when there is no .gitignore file in the directory.
        assert_eq!(lookup(&root_dir_path, false), root_dir_path.join(".gitignore"));

        // Assert that searching for a .gitignore file path when [only_git_root] is [false]
        // will return a path when there is a .gitignore file in the directory.
        fs::File::create(root_dir_path.join(".gitignore")).unwrap();
        assert_eq!(lookup(&root_dir_path, false), root_dir_path.join(".gitignore"));

        // Assert that searching for a .gitignore file path when [only_git_root] is true
        // will not return a path if there is no .git directory, even if there is a .gitignore
        // file.
        assert!(catch_unwind(|| lookup(&root_dir_path, true)).is_err());

        // Assert that searching for a .gitignore file path when [only_git_root] is true
        // will return a path if there is a .git directory, when there is no .gitignore
        // file.
        fs::create_dir(root_dir_path.join(".git")).unwrap();
        fs::remove_file(root_dir_path.join(".gitignore")).unwrap();
        assert_eq!(lookup(&root_dir_path, true), root_dir_path.join(".gitignore"));

        // Assert that searching for a .gitignore file path when [only_git_root] is true
        // will return a path if there is a .git directory, when there is a .gitignore
        // file.
        fs::File::create(root_dir_path.join(".gitignore")).unwrap();
        assert_eq!(lookup(&root_dir_path, true), root_dir_path.join(".gitignore"));

        // Assert that searching for a .gitignore file when [only_git_root] is true
        // will return a path if a parent dir has a .git directory, when there is no .gitignore
        // file.
        let sub_dir_path = root_dir_path.join("subdir");
        fs::create_dir(sub_dir_path.clone()).unwrap();
        fs::File::create(sub_dir_path.join("sub.txt")).unwrap();
        fs::remove_file(root_dir_path.join(".gitignore")).unwrap();
        assert_eq!(lookup(&sub_dir_path, true), root_dir_path.join(".gitignore"));

        // Assert that searching for a .gitignore file when [only_git_root] is true
        // will return a path if a parent dir has a .git directory, when there is a .gitignore
        // file.
        fs::File::create(root_dir_path.join(".gitignore")).unwrap();
        assert_eq!(lookup(&sub_dir_path, true), root_dir_path.join(".gitignore"));

        root_dir.close().unwrap();
    }
}

