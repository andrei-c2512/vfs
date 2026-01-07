use std::cmp::min;
use std::io::{Seek,SeekFrom,Write,Read};
use std::fs;
use std::cell::RefCell;
use std::rc::Rc;

use crate::traits::Serde;
use crate::fs_base::Error;
use crate::serde;
use crate::fs_base;


pub struct BlockDevice{
    free_blocks : Vec<u32>,

    // we write a maximum of 8KB
    buffer : Vec<u8>,
    // the number of blocks we allocated 
    num_blocks : u32,
}

impl BlockDevice{
    // overwrites the provided blocks with the provided buffer
    pub fn new() -> Self {
        Self{free_blocks : Vec::new(), buffer : Vec::new(), num_blocks : 0u32}
    }
    pub fn from(free_blocks : Vec<u32>, num_blocks : u32) -> Self{
        Self{free_blocks, buffer : Vec::new(), num_blocks}
    }
    // this function does not do any release build bounds checking!!!
    pub fn read_to_string(&mut self, data : &mut String, block_indices : &[u32], size : usize, file : Rc<RefCell<fs::File>>){
       assert!(size <= block_indices.len() * fs_base::BLOCK_CAPACITY);

       let mut left_to_read = size;
       for block_id in block_indices.iter() { 
           let to_read = min(left_to_read, fs_base::BLOCK_CAPACITY);
           self.load_block(*block_id, file.clone(), to_read);
           // --- REWRITE: Find a solution with less copies. This is so fuckin ass. Why can t I just check if it's utf8 without copies?
           // edit: Since I already do buffered reading, maybe have a String buffer instead of a
           // byte buffer ?
           let string_block = String::from_utf8(self.buffer.clone()).unwrap();
           data.push_str(&string_block);
           left_to_read -= to_read;

           if left_to_read == 0 {
               break;
           }
       }
    }
    pub fn read(&mut self, data : &mut Vec<u8>, block_indices : &[u32], size : usize, file : Rc<RefCell<fs::File>>) {
       assert!(size <= block_indices.len() * fs_base::BLOCK_CAPACITY);

       let mut left_to_read = size;
       for block_id in block_indices.iter() { 
           let to_read = min(left_to_read, fs_base::BLOCK_CAPACITY);
           self.load_block(*block_id, file.clone(), to_read);
           data.extend_from_slice(&self.buffer);
           left_to_read -= to_read;

           if left_to_read == 0 {
               break;
           }
       }
    }
    pub fn write(&mut self, buffer :  &[u8], block_indices : &mut Vec<u32>, file : Rc<RefCell<fs::File>>) {
        // --- REWRITE: the header size should be expandable, not hardcoded
        let header_offset = fs_base::HEADER_SIZE;

        let mut slice = buffer;
        for index in block_indices.iter() {
            let mut file_ref = file.borrow_mut();
            if buffer.is_empty() {
                return;
            }
            // --- REWRITE: handle this error
            let _= file_ref.seek(SeekFrom::Start(
                    (header_offset + (*index as usize) * fs_base::BLOCK_CAPACITY) as u64)
                );
            let write_len = min(fs_base::MAX_PROCESS_CAPACITY, buffer.len());
            let to_write = &buffer[0..write_len];
            slice = &slice[write_len..];

            // --- REWRITE: handle this error
            let _ = file_ref.write_all(to_write);
        }

        if !buffer.is_empty(){
            let new_blocks = &Self::append(buffer, file.clone());
            for i in 0..(*new_blocks) {
                block_indices.push(i + self.num_blocks);
            }
            self.num_blocks += new_blocks; 
        }
    }     
    // returns the number of blocks that been appended to the file
    pub fn append(buffer : &[u8], file : Rc<RefCell<fs::File>>) -> u32{
        let blocks = Self::buffer_in_blocks(buffer);
        // --- REWRITE: handle this error vro. This MIGHT result in a file corruption: what if 
        // write_all() fails and I return the wrong number of blocks that have been written? Might
        // mess up everything
        {
            // --- REWRITE: handle this error vro
            let mut file_ref = file.borrow_mut();
            let _ = file_ref.seek(SeekFrom::End(0));
            let _ = file_ref.write_all(buffer);
        }

        Self::append_block_remainder(buffer.len(), file); 

        blocks
    }
    fn load_block(&mut self, index : u32, file : Rc<RefCell<fs::File>>, len : usize){
        // again, I do a pointless memset I believe. Why can't I just change size and overwrite
        // directly bruh
        self.buffer.resize(len, 0);
        let mut file_ref = file.borrow_mut();
        // --- REWRITE: Handle errors vro
        let _ = file_ref.seek(SeekFrom::Start((fs_base::HEADER_SIZE + index as usize * fs_base::BLOCK_CAPACITY) as u64));
        if let Err(err) = file_ref.read_exact(&mut self.buffer){
            println!("Error in loading block: {}", err);
        }
    }
    fn buffer_in_blocks(buffer : &[u8]) -> u32{
        let mut blocks = buffer.len() / fs_base::BLOCK_CAPACITY;
        if buffer.len().is_multiple_of(fs_base::BLOCK_CAPACITY) {
            blocks += 1;
        }
        blocks as u32
    }

    pub fn append_block_remainder(buffer_len : usize, file : Rc<RefCell<fs::File>>){
        let remainder = fs_base::BLOCK_CAPACITY - buffer_len % fs_base::BLOCK_CAPACITY; 
        if remainder == 0 { return; }

        let blank = vec![b'_'; remainder];

        // --- REWRITE: handle this error vro
        {
            let mut file_ref = file.borrow_mut();
            let _ = file_ref.seek(SeekFrom::End(0));
            let _ =  file_ref.write_all(&blank);
        }
    }
    pub fn append_header_filler(buffer_len : usize, file : Rc<RefCell<fs::File>>){
        let remainder = fs_base::HEADER_SIZE - buffer_len - fs_base::HEADER_TAIL.len(); 
        if remainder == 0 { return; }

        let blank = vec![b'-'; remainder];


        // --- REWRITE: handle this error vro
        {
            let mut file_ref = file.borrow_mut();

            match file_ref.seek(SeekFrom::Start(buffer_len as u64)){
                Ok(_) => {
                }
                Err(err) => {
                    println!("Error in seeking to the end of the file: {}", err);
                }
            }
            match file_ref.write_all(&blank) {
                Ok(_) => {
                }
                Err(err) => {
                    println!("Error in writing to the end of the file: {}", err);
                }
            }

            let _ = file_ref.seek(SeekFrom::Start((buffer_len + remainder)as u64));
            let _ =  file_ref.write_all(fs_base::HEADER_TAIL.as_bytes());
        }
    }
}

impl Serde for BlockDevice {
    fn serialize(&self) -> Vec<u8>{
        let mut res = Vec::new();
        res.extend_from_slice(
            fs_base::BLOCK_DEVICE_PREAMBLE.as_bytes()
        );
        res.extend_from_slice(
            &serde::ser_vec_u32(&self.free_blocks)
        );
        res.extend_from_slice(
            &self.num_blocks.to_be_bytes()
        );
        res
    }
    fn deserialize(buffer : &mut &[u8]) -> Result<Self, Error>{
        if !buffer.starts_with(fs_base::BLOCK_DEVICE_PREAMBLE.as_bytes()) {
            return Err(Error::InvalidPreamble("Did not find the preamble for block device".to_string()));
        }
        *buffer = &buffer[fs_base::BLOCK_DEVICE_PREAMBLE.len()..];
        let free_blocks = serde::deser_vec_u32(buffer)?;
        let n_blocks = serde::deser_u32(buffer)?;
        
        Ok(BlockDevice::from(free_blocks, n_blocks))
    }
}
impl Default for BlockDevice{
    fn default() -> Self{
        Self::new()
    }
}
