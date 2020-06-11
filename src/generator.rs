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

    fn get(&self, index: usize) -> &Block { &self.vec[index] }

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

pub fn generate(using: &Vec<PathBuf>, into: &PathBuf) {
    let into_contents = fs::read_to_string(into.clone()).unwrap();
    let mut into_lines: Vec<String> = into_contents.lines().map(|s| String::from(s)).collect();

    let mut block_vec = BlockVec::from(&into_lines);

    for path in using {
        let new_contents = fs::read_to_string(path.clone()).unwrap();
        let new_lines: Vec<&str> = new_contents.lines().collect();

        let file_stem = String::from(path.file_stem().unwrap().to_str().unwrap());
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
        }
    }
}

