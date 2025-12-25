use std::collections::HashMap;
use std::cmp::min;
use std::io::{Seek,SeekFrom, Write};
use std::fs;
use std::cell::RefCell;
use std::rc::Rc;

use crate::traits::Serde;
use crate::fs_base::Error;
use crate::serde;

pub const BLOCK_CAPACITY : usize = 1024 * 4;
pub const MAX_PROCESS_CAPACITY : usize= BLOCK_CAPACITY * 2;

pub struct BlockDevice{
    free_blocks : Vec<u32>,

    // we write a maximum of 8KB
    buffer : [u8; 2 * BLOCK_CAPACITY],
    // we load a maximum of 2 blocks
    loaded_blocks : [u32; 2],
    // the number of blocks we allocated 
    num_blocks : u32,
}

impl BlockDevice{
    // overwrites the provided blocks with the provided buffer
    pub fn new() -> Self {
        Self{free_blocks : Vec::new(), buffer : [0u8; MAX_PROCESS_CAPACITY], loaded_blocks : [0u32, 2], num_blocks : 0u32}
    }
    pub fn from(free_blocks : Vec<u32>, num_blocks : u32) -> Self{
        Self{free_blocks : free_blocks, buffer : [0u8; MAX_PROCESS_CAPACITY], loaded_blocks : [0u32, 2], num_blocks : num_blocks}
    }
    pub fn write(&self, buffer :  &[u8], block_indices : &mut Vec<u32>, file : Rc<RefCell<fs::File>>) {
        // --- REWRITE: the header size should be expandable, not hardcoded
        let header_offset = BLOCK_CAPACITY;
        let mut file_ref = file.borrow_mut();

        let mut slice = buffer;
        for index in block_indices.iter() {
            if buffer.len() == 0 {
                return;
            }
            file_ref.seek(SeekFrom::Start(
                    (header_offset + (*index as usize) * BLOCK_CAPACITY) as u64)
                );
            let write_len = min(MAX_PROCESS_CAPACITY, buffer.len());
            let to_write = &buffer[0..write_len];
            slice = &slice[write_len..];

            file_ref.write_all(to_write);
        }

        if buffer.len() > 0 {
            let new_blocks = &Self::append(buffer, file.clone());
            for i in self.num_blocks..(*new_blocks) {
                block_indices.push(i);
            }
        }
    }    
    
    // returns the number of blocks that been appended to the file
    pub fn append(buffer : &[u8], file : Rc<RefCell<fs::File>>) -> u32{
        let blocks = Self::buffer_in_blocks(buffer);
        file.borrow_mut().write_all(buffer);
        Self::append_block_remainder(buffer.len(), file);

        blocks
    }

    fn buffer_in_blocks(buffer : &[u8]) -> u32{
        let mut blocks = buffer.len() / BLOCK_CAPACITY;
        if buffer.len() % BLOCK_CAPACITY != 0 {
            blocks += 1;
        }
        return blocks as u32;
    }
    fn append_block_remainder(buffer_len : usize, file : Rc<RefCell<fs::File>>){
        let remainder = buffer_len % BLOCK_CAPACITY; 
        if remainder == 0 { return; }

        let blank = vec![0u8; remainder];
        file.borrow_mut().write_all(&blank);
    }
}

impl Serde for BlockDevice {
    fn serialize(&self) -> Vec<u8>{
        let mut res = Vec::new();
        res.extend_from_slice(
            &serde::ser_vec_u32(&self.free_blocks)
            );
        res.extend_from_slice(
            &self.num_blocks.to_be_bytes()
        );
        res
    }
    fn deserialize(buffer : &mut &[u8]) -> Result<Self, Error>{
        let free_blocks = serde::deser_vec_u32(buffer)?;
        let n_blocks = serde::deser_u32(buffer)?;
        
        Ok(BlockDevice::from(free_blocks, n_blocks))
    }
}
