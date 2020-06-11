//! .gitignore content generation

use std::fs;
use std::path::PathBuf;

static BLOCK_PREFIX: &str = "# GITIGNORE-CLI/";
static BLOCK_START: &str = "START:";
static BLOCK_END: &str = "END:";

struct Block<'a> {
    name: &'a str,
    start: usize,
    size: usize
}

struct BlockVec<'a> {
    vec: Vec<Block<'a>>
}

impl<'a> BlockVec<'a> {

    fn from(lines: &Vec<&'a str>) -> BlockVec<'a> {
        let mut bv = BlockVec { vec: Vec::new() };

        let mut block_name: Option<&'a str> = None;
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
                        name: block_name.unwrap(),
                        start: block_start.unwrap(),
                        size: index - block_start.unwrap()
                    });

                    block_start = None;
                    block_name = None;
                }
            }
        }

        bv
    }
}

pub fn generate(using: &Vec<PathBuf>, into: &PathBuf) {
    let into_file_contents = fs::read_to_string(into.clone()).unwrap();
    let mut lines: Vec<&str> = into_file_contents.lines().collect();

    let block_vec = BlockVec::from(&lines);
}

