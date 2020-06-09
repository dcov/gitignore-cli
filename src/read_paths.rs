//! Read paths lookup.
//!
//! This module contains the lookup functionality for the '*.gitignore' files from
//! which the contents will be read to generate the resulting '.gitignore' file.
//!
//! Note: It only looks up the paths of the files, and returns them. It does not
//! open any files.

use std::fs;
use std::path::PathBuf;

fn path_is_valid(path: &PathBuf, with: &Vec<&str>) -> bool {
    !path.is_dir()
    && match path.extension() {
        Some(extension) => extension == "gitignore",
        None => false
    }
    && match path.file_stem() {
        Some(stem) => {
            let mut result = false;

            let stem = stem.to_str().unwrap().to_ascii_lowercase();
            for with_entry in with {
                if with_entry.to_ascii_lowercase() == stem {
                    result = true;
                    break;
                }
            }

            result
        },
        None => false
    }
}

pub fn lookup<'a>(from: &PathBuf, with: &Vec<&str>) -> Vec<PathBuf> {
    let from_dir = fs::read_dir(from).expect(
        format!("Could not read from {}", from.to_str().unwrap()).as_str());

    let mut read_files: Vec<PathBuf> = Vec::new();

    for entry_result in from_dir {
        let entry_path = entry_result.expect("Could not check a dir entry.").path();
        if path_is_valid(&entry_path, &with) {
            read_files.push(entry_path);
        }
    }

    return read_files;
}

#[cfg(test)]
mod tests {

    use super::*;
    use tempfile;

    fn contain_same_files(left: Vec<PathBuf>, right: &mut Vec<PathBuf>) -> bool {
        if left.len() != right.len() {
            return false;
        }

        for path in left {
            let mut remove_index: Option<usize> = None;
            for (i, v) in right.iter().enumerate() {
                if *v == path {
                    remove_index = Some(i);
                    break;
                }
            };

            if remove_index.is_none() {
                return false;
            }

            right.remove(remove_index.unwrap());
        }

        true
    }

    #[test]
    fn test_lookup() {
        let dir = tempfile::tempdir().unwrap();
        let dir_path = dir.path().to_path_buf();

        let with = vec!["dir", "rust", "Lua", "HASKELL", "java"];

        // Assert that [lookup] returns an empty list when the directory doesn't
        // contain any files that match the '*.gitignore' pattern.
        fs::File::create(dir_path.join(".gitignore")).unwrap();
        fs::File::create(dir_path.join("fizz.txt")).unwrap();
        fs::File::create(dir_path.join("buzz.c")).unwrap();
        assert!(lookup(&dir_path, &with).is_empty());

        // Assert that [lookup] ignores directories that match the pattern
        fs::create_dir(dir_path.join("dir.gitignore")).unwrap();
        assert!(lookup(&dir_path, &with).is_empty());

        // Assert that [lookup] will return the files that match the pattern and
        // are included in the [with] list.
        fs::File::create(dir_path.join("rust.gitignore")).unwrap();
        fs::File::create(dir_path.join("java.gitignore")).unwrap();
        assert!(contain_same_files(
                    lookup(&dir_path, &with),
                    &mut vec![dir_path.join("rust.gitignore"), dir_path.join("java.gitignore")]));

        // Assert that [lookup] will ignore character casing in both file names and
        // the values in the [with] list.
        fs::File::create(dir_path.join("LUA.gitignore")).unwrap();
        fs::File::create(dir_path.join("haskeLL.gitignore")).unwrap();
        assert!(contain_same_files(
                    lookup(&dir_path, &with),
                    &mut vec![dir_path.join("rust.gitignore"), dir_path.join("java.gitignore"),
                              dir_path.join("LUA.gitignore"), dir_path.join("haskeLL.gitignore")]));

        dir.close().unwrap();
    }
}

