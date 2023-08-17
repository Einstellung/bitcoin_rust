use crate::block::Block;

pub struct Blockchain {
    blocks: Vec<Block>
}

impl Blockchain {
    pub fn new() -> Blockchain {
        let genesis_block = Block::new_genesis_block();
        Blockchain { blocks: vec![genesis_block] }
    }

    pub fn add_block(&mut self, data: &str) {
        let prev_block = self.blocks.last().unwrap().clone();
        let new_block = Block::new(data, prev_block.hash.clone());
        self.blocks.push(new_block);
    }

    pub fn print_block(&self) {
        for block in &self.blocks {
            block.print_content();
            println!("------------");
        }
    }
}