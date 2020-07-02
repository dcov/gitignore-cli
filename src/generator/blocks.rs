

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

    fn len(&self) -> usize {
        self.vec.len()
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
