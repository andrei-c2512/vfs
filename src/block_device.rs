
pub const BLOCK_CAPACITY = 1024 * 4;

struct BlockDevice{
    blocks : Vec<[u8;BLOCK_CAPACITY]>,
}

impl BlockDevice{
    pub fn new(num_blocks: usize) -> Self{
        Self{ blocks: vec![[0u8; BLOCK_CAPACITY]; num_blocks] }
    }
    pub fn read(&self, index : usize) -> &[u8;BLOCK_CAPACITY] {
        self.blocks[index]
    }
    pub fn write(&mut self, index : usize, buffer : &[u8; BLOCK_CAPACITY]){
        self.blocks[index] = *buffer;
    }
}
