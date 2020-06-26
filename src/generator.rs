//! .gitignore content generation

use std::fs;
use std::path::PathBuf;

static BLOCK_PREFIX: &str = "# GITIGNORE-CLI/";
static BLOCK_START: &str = "START:";
static BLOCK_END: &str = "END:";

struct Block {
    name: String,
    start: usize,
    size: usize
}

struct BlockVec {
    vec: Vec<Block>
}

impl<'a> BlockVec {

    fn get(&self, index: usize) -> &Block {
        &self.vec[index]
    }

    fn remove(&mut self, index: usize) -> Block {
        self.vec.remove(index)
    }

    fn from(lines: &Vec<String>) -> BlockVec {
        let mut bv = BlockVec { vec: Vec::new() };

        let mut block_name: Option<&str> = None;
        let mut block_start: Option<usize> = None;
        for (index, line) in lines.iter().enumerate() {
            if line.starts_with(BLOCK_PREFIX) {
                let block_header = line.split_at(BLOCK_PREFIX.len()).1;
                if block_header.starts_with(BLOCK_START) {
                    debug_assert!(block_start.is_none());
                    debug_assert!(block_name.is_none());

                    block_start = Some(index);

                    let starting_block_name = block_header.split_at(BLOCK_START.len()).1;
                    debug_assert!(!starting_block_name.is_empty());

                    block_name = Some(starting_block_name);
                } else {
                    debug_assert!(block_header.starts_with(BLOCK_END));
                    debug_assert!(block_start.is_some());
                    debug_assert!(block_name.is_some());

                    let ending_block_name = block_header.split_at(BLOCK_END.len()).1;
                    debug_assert_eq!(ending_block_name, block_name.unwrap());

                    bv.vec.push(Block {
                        name: String::from(block_name.unwrap()),
                        start: block_start.unwrap(),
                        size: index - block_start.unwrap() - 1
                    });

                    block_start = None;
                    block_name = None;
                }
            }
        }

        bv
    }

    fn index_of(&self, name: &String) -> Option<usize> {
        for (index, block) in self.vec.iter().enumerate() {
            if block.name == *name {
                return Some(index);
            }
        }
        None
    }

    fn shift_starts_up(&mut self, from: usize, by: usize) {
        for i in from..self.vec.len() {
            let block = &mut self.vec[i];
            block.start += by;
        }
    }

    fn shift_starts_down(&mut self, from: usize, by: usize) {
        for i in from..self.vec.len() {
            let block = &mut self.vec[i];
            block.start -= by;
        }
    }
}

pub fn insert(into: &PathBuf, using: &Vec<PathBuf>) {
    let into_contents = fs::read_to_string(into.clone()).unwrap_or(String::from(""));
    let mut into_lines: Vec<String> = into_contents.lines().map(String::from).collect();

    let mut block_vec = BlockVec::from(&into_lines);

    for path in using {
        let new_contents = fs::read_to_string(path.clone()).unwrap();
        let new_lines: Vec<&str> = new_contents.lines().collect();

        let file_stem = String::from(path.file_stem().unwrap().to_str().unwrap()).to_ascii_lowercase();
        if let Some(block_index) = block_vec.index_of(&file_stem) {
            let block = block_vec.get(block_index);

            let remove_index = block.start + 1;
            for _ in 0..block.size {
                into_lines.remove(remove_index);
            }

            let initial_insert_index = remove_index;
            for (index, line) in new_lines.iter().enumerate() {
                into_lines.insert(initial_insert_index + index, String::from(*line));
            }

            let size_diff = (new_lines.len() as i8) - (block.size as i8);
            if size_diff > 0 {
                block_vec.shift_starts_up(block_index + 1, size_diff as usize);
            } else if size_diff < 0 {
                block_vec.shift_starts_down(block_index + 1, size_diff.abs() as usize);
            }
        } else {
            if !into_lines.is_empty() && !into_lines.last().unwrap().is_empty() {
                // Push an empty line in between the last line and the lines we're going to add
                into_lines.push(String::from(""));
            }

            into_lines.push(format!("{}{}{}", BLOCK_PREFIX, BLOCK_START, file_stem));
            for line in new_lines {
                into_lines.push(String::from(line));
            }
            into_lines.push(format!("{}{}{}", BLOCK_PREFIX, BLOCK_END, file_stem));
            into_lines.push(String::from(""));
        }
    }

    let result = into_lines.join("\n");
    fs::write(into, result.as_bytes())
        .expect(format!("Failed to write result to {}", into.to_str().unwrap()).as_str());
}

pub fn remove(from: &PathBuf, using: &Vec<String>) {
    let from_contents = fs::read_to_string(from.clone())
        .expect(format!("{} does not exist, or is empty", from.to_str().unwrap()).as_str());
    let mut from_lines: Vec<String> = from_contents.lines().map(String::from).collect();
    let mut block_vec = BlockVec::from(&from_lines);

    for name in using {
        if let Some(block_index) = block_vec.index_of(&name.to_ascii_lowercase()) {
            // Remove the block from the list of blocks
            let block = block_vec.remove(block_index);

            // Remove the block's lines
            from_lines.remove(block.start);
            for _ in 0..block.size {
                from_lines.remove(block.start);
            }
            from_lines.remove(block.start);

            // Shift the remaining blocks down by the block size + 2
            block_vec.shift_starts_down(block_index, block.size + 2);
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use cascade;
    use tempfile;

    fn format_as_block(stem: &str, contents: &str) -> String {
        format!("{}{}{}\n{}\n{}{}{}",
            BLOCK_PREFIX, BLOCK_START, stem,
            contents,
            BLOCK_PREFIX, BLOCK_END, stem)
    }

    #[test]
    fn test_insert() {
        let dir = tempfile::tempdir().unwrap();
        let dir_path = dir.path();
        let write_path = dir_path.join("write.gitignore");


        // Assert that [insert] correctly formats and writes the contents.
        let rust_path = dir_path.join("rust.gitignore");
        let rust_contents = "target/\nCargo.lock";
        fs::write(rust_path.clone(), rust_contents).unwrap();
        let rust_block = format_as_block("rust", rust_contents); // The expected block formatting.
        insert(&write_path, &vec![rust_path.clone()]);
        assert_eq!(
            fs::read_to_string(write_path.clone()).unwrap(),
            cascade!{
                rust_block.clone();
                ..push_str("\n");
            });


        // Assert that [insert] correctly formats and appends the contents.
        let python_path = dir_path.join("python.gitignore");
        let python_contents = "build/\ndist/";
        fs::write(python_path.clone(), python_contents).unwrap();
        let python_block = format_as_block("python", python_contents);
        insert(&write_path, &vec![python_path.clone()]);
        assert_eq!(
            fs::read_to_string(write_path.clone()).unwrap(),
            cascade! {
                rust_block.clone();
                ..push_str("\n\n");
                ..push_str(&python_block);
                ..push_str("\n");
            });


        // Assert that [insert] correctly edits the contents when existing blocks' contents
        // have changed.
        let rust_contents = "target/";
        fs::write(rust_path.clone(), rust_contents).unwrap();
        let rust_block = format_as_block("rust", rust_contents);
        insert(&write_path, &vec![rust_path.clone()]);
        assert_eq!(
            fs::read_to_string(write_path.clone()).unwrap(),
            cascade! {
                rust_block.clone();
                ..push_str("\n\n");
                ..push_str(&python_block);
            });


        // Assert that [insert] doesn't unintentionally change anything when existing blocks'
        // contents haven't changed.
        insert(&write_path, &vec![rust_path.clone(), python_path.clone()]);
        assert_eq!(
            fs::read_to_string(write_path.clone()).unwrap(),
            cascade! {
                rust_block.clone();
                ..push_str("\n\n");
                ..push_str(&python_block);
            });


        dir.close().unwrap();
    }
}

