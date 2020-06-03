//! Write file lookup.
//!
//! This module contains the lookup functionality for the '.gitignore' file that
//! will be written to.

use std::fs::{self, File, DirEntry};
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

fn find_gitignore_path(from_dir_path: PathBuf, only_git_root: bool) -> Option<PathBuf> {
    let mut current_dir_path = from_dir_path;
    loop {
        if let Some(path) = search_dir_for_gitignore_path(&current_dir_path, only_git_root) {
            return Some(path);
        }

        if !only_git_root || !current_dir_path.pop() {
            break;
        }
    }

    None
}

/// Opens and returns the .gitignore file that should be written to.
pub fn open(from_dir_path: PathBuf, only_git_root: bool) -> Result<File, ()> {
    if let Some(path) = find_gitignore_path(from_dir_path, only_git_root) {
        if let Ok(file) = File::open(path.clone()) {
            return Ok(file);
        }

        if let Ok(file) = File::create(path.clone()) {
            return Ok(file);
        }
    }

    Err(())
}

#[cfg(test)]
mod tests {

    use super::*;
    use tempfile;

    #[test]
    fn test_find_gitignore_path() {
        let root_dir = tempfile::tempdir().unwrap();
        let root_dir_path = root_dir.path().to_path_buf();

        // Add some random files into the directory
        File::create(root_dir_path.join("random.txt")).unwrap();
        File::create(root_dir_path.join("fizz.h")).unwrap();
        File::create(root_dir_path.join("buzz.c")).unwrap();

        // Assert that searching for a .gitignore file path when [only_git_root] is [false]
        // will return a path when there is no .gitignore file in the directory.
        let result = find_gitignore_path(root_dir_path.clone(), false);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), root_dir_path.join(".gitignore"));

        // Assert that searching for a .gitignore file path when [only_git_root] is [false]
        // will return a path when there is a .gitignore file in the directory.
        File::create(root_dir_path.join(".gitignore")).unwrap();
        let result = find_gitignore_path(root_dir_path.clone(), false);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), root_dir_path.join(".gitignore"));

        // Assert that searching for a .gitignore file path when [only_git_root] is true
        // will not return a path if there is no .git directory, even if there is a .gitignore
        // file.
        let result = find_gitignore_path(root_dir_path.clone(), true);
        assert!(result.is_none());

        // Assert that searching for a .gitignore file path when [only_git_root] is true
        // will return a path if there is a .git directory, when there is no .gitignore
        // file.
        fs::create_dir(root_dir_path.join(".git")).unwrap();
        fs::remove_file(root_dir_path.join(".gitignore")).unwrap();
        let result = find_gitignore_path(root_dir_path.clone(), true);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), root_dir_path.join(".gitignore"));

        // Assert that searching for a .gitignore file path when [only_git_root] is true
        // will return a path if there is a .git directory, when there is a .gitignore
        // file.
        File::create(root_dir_path.join(".gitignore")).unwrap();
        let result = find_gitignore_path(root_dir_path.clone(), true);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), root_dir_path.join(".gitignore"));

        // Assert that searching for a .gitignore file when [only_git_root] is true
        // will return a path if a parent dir has a .git directory, when there is no .gitignore
        // file.
        let sub_dir_path = root_dir_path.join("subdir");
        fs::create_dir(sub_dir_path.clone()).unwrap();
        File::create(sub_dir_path.join("sub.txt")).unwrap();
        fs::remove_file(root_dir_path.join(".gitignore")).unwrap();
        let result = find_gitignore_path(sub_dir_path.clone(), true);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), root_dir_path.join(".gitignore"));

        // Assert that searching for a .gitignore file when [only_git_root] is true
        // will return a path if a parent dir has a .git directory, when there is a .gitignore
        // file.
        File::create(root_dir_path.join(".gitignore")).unwrap();
        let result = find_gitignore_path(sub_dir_path.clone(), true);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), root_dir_path.join(".gitignore"));

        root_dir.close().unwrap();
    }
}

