use std::cell::RefCell;
use std::cmp::min;
use std::fs;
use std::io::{Read, Seek, SeekFrom, Write};
use std::rc::Rc;

use crate::fs_base;
use crate::fs_base::Error;
use crate::serde;
use crate::traits::Serde;

pub struct BlockDevice {
    free_blocks: Vec<u32>,

    // we write a maximum of 8KB
    buffer: Vec<u8>,
    // the number of blocks we allocated
    num_blocks: u32,
}

impl BlockDevice {
    // overwrites the provided blocks with the provided buffer
    pub fn new() -> Self {
        Self {
            free_blocks: Vec::new(),
            buffer: Vec::new(),
            num_blocks: 0u32,
        }
    }
    pub fn from(free_blocks: Vec<u32>, num_blocks: u32) -> Self {
        Self {
            free_blocks,
            buffer: Vec::new(),
            num_blocks,
        }
    }
    // this function does not do any release build bounds checking!!!
    pub fn read_to_string(
        &mut self,
        data: &mut String,
        block_indices: &[u32],
        size: usize,
        file: Rc<RefCell<fs::File>>,
    ) {
        // println!("Size: {}, To read: {}", size, block_indices.len() * fs_base::BLOCK_CAPACITY);
        assert!(size <= block_indices.len() * fs_base::BLOCK_CAPACITY);
        assert!(!block_indices.is_empty());

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
    pub fn read(
        &mut self,
        data: &mut Vec<u8>,
        block_indices: &[u32],
        size: usize,
        file: Rc<RefCell<fs::File>>,
    ) {
        //println!("Size: {}, To read: {}", size, block_indices.len() * fs_base::BLOCK_CAPACITY);
        assert!(size <= block_indices.len() * fs_base::BLOCK_CAPACITY);
        assert!(!block_indices.is_empty());

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

    pub fn read_from(
        &mut self,
        data: &mut [u8; fs_base::BUFFERED_IO_LIMIT],
        block_indices: &[u32],
        file: Rc<RefCell<fs::File>>,
        file_size: usize,
        from: &mut u32,
    ) -> Result<usize, Error> {
        assert!(
            *from as usize * fs_base::BLOCK_CAPACITY
                <= block_indices.len() * fs_base::BLOCK_CAPACITY
        );
        assert!(!block_indices.is_empty());

        let total_to_read = {
            let max_future_blocks_read = fs_base::BUFFERED_IO_LIMIT / fs_base::BLOCK_CAPACITY;
            if *from as usize + max_future_blocks_read >= block_indices.len() {
                file_size % fs_base::BUFFERED_IO_LIMIT
            } else {
                fs_base::BUFFERED_IO_LIMIT
            }
        };
        let mut left_to_read = total_to_read;

        let mut start = 0;
        for block_id in block_indices.iter().skip(*from as usize) {
            let to_read = min(left_to_read, fs_base::BLOCK_CAPACITY);
            Self::load_block_into(&mut data[start..start + to_read], *block_id, file.clone());
            //data.extend_from_slice(&self.buffer);
            left_to_read -= to_read;
            start += to_read;
            *from += 1;
            if left_to_read == 0 {
                break;
            }
        }

        Ok(start)
    }
    pub fn write(
        &mut self,
        buffer: &[u8],
        block_indices: &mut Vec<u32>,
        file: Rc<RefCell<fs::File>>,
    ) {
        self.write_from(buffer, block_indices, file, 0, 0)
    }
    /*
     * Flow:
     *  Case 1:
     *      The file is empty and doesn't have any capacity, so we just allocate (works)
     *  Case 2:
     *      The file is being appended:
     *          a) The file doesn't have capacity -> get's allocations
     *          b)
     */
    fn write_from(
        &mut self,
        buffer: &[u8],
        block_indices: &mut Vec<u32>,
        file: Rc<RefCell<fs::File>>,
        block_ind: usize,
        vfs_file_size: usize,
    ) {
        // --- REWRITE: the header size should be expandable, not hardcoded
        let header_offset = fs_base::HEADER_SIZE;
        //println!("Writing a buffer of length: {}", buffer.len());

        let mut slice = buffer;
        let block_ind_start = {
            let file_offset = vfs_file_size % fs_base::BLOCK_CAPACITY;
            if self.write_block_remainder(&mut slice, block_ind as u32, file.clone(), file_offset) {
                block_ind + 1
            } else {
                block_ind
            }
        };
        for index in block_indices.iter().skip(block_ind_start) {
            let mut file_ref = file.borrow_mut();
            if slice.is_empty() {
                return;
            }
            // --- REWRITE: handle this error
            let _ = file_ref.seek(SeekFrom::Start(
                (header_offset + (*index as usize) * fs_base::BLOCK_CAPACITY) as u64,
            ));
            let write_len = min(fs_base::MAX_PROCESS_CAPACITY, slice.len());
            let to_write = &slice[0..write_len];
            slice = &slice[write_len..];

            // --- REWRITE: handle this error
            let _ = file_ref.write_all(to_write);
        }

        if !slice.is_empty() {
            // println!("Left to append: {}", slice.len());
            let new_blocks = Self::allocate(slice, file.clone());
            for i in 0..(new_blocks) {
                block_indices.push(i + self.num_blocks);
            }
            self.num_blocks += new_blocks;
        }
    }
    // returns the number of blocks that been appended to the file
    pub fn allocate(buffer: &[u8], file: Rc<RefCell<fs::File>>) -> u32 {
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
    pub fn append(
        &mut self,
        buffer: &[u8],
        block_indices: &mut Vec<u32>,
        file: Rc<RefCell<fs::File>>,
        vfs_file_size: usize,
    ) {
        let unfilled_block_index = Self::size_in_blocks(vfs_file_size);
        println!("Unfilled block index: {}", unfilled_block_index);
        println!("Vfs file size: {}", vfs_file_size);
        self.write_from(
            buffer,
            block_indices,
            file,
            unfilled_block_index as usize,
            vfs_file_size,
        );
    }
    fn load_block(&mut self, index: u32, file: Rc<RefCell<fs::File>>, len: usize) {
        self.buffer.resize(len, 0);
        Self::load_block_into(&mut self.buffer, index, file.clone());
    }
    fn load_block_into(buffer: &mut [u8], index: u32, file: Rc<RefCell<fs::File>>) {
        // again, I do a pointless memset I believe. Why can't I just change size and overwrite
        // directly bruh
        let mut file_ref = file.borrow_mut();
        // --- REWRITE: Handle errors vro
        let _ = file_ref.seek(SeekFrom::Start(
            (fs_base::HEADER_SIZE + index as usize * fs_base::BLOCK_CAPACITY) as u64,
        ));
        if let Err(err) = file_ref.read_exact(buffer) {
            println!("Error in loading block: {}", err);
        }
    }

    fn size_in_blocks(size: usize) -> u32 {
        let mut blocks = size / fs_base::BLOCK_CAPACITY;
        if !size.is_multiple_of(fs_base::BLOCK_CAPACITY) {
            blocks += 1;
        }
        blocks as u32
    }
    fn buffer_in_blocks(buffer: &[u8]) -> u32 {
        Self::size_in_blocks(buffer.len())
    }
    pub fn append_block_remainder(buffer_len: usize, file: Rc<RefCell<fs::File>>) {
        let remainder = fs_base::BLOCK_CAPACITY - buffer_len % fs_base::BLOCK_CAPACITY;
        if remainder == 0 {
            return;
        }

        let blank = vec![b'_'; remainder];

        // --- REWRITE: handle this error vro
        {
            let mut file_ref = file.borrow_mut();
            let _ = file_ref.seek(SeekFrom::End(0));
            let _ = file_ref.write_all(&blank);
        }
    }
    pub fn append_header_filler(buffer_len: usize, file: Rc<RefCell<fs::File>>) {
        let remainder = fs_base::HEADER_SIZE - buffer_len - fs_base::HEADER_TAIL.len();
        if remainder == 0 {
            return;
        }

        let blank = vec![b'-'; remainder];

        // --- REWRITE: handle this error vro
        {
            let mut file_ref = file.borrow_mut();

            if let Err(err) = file_ref.seek(SeekFrom::Start(buffer_len as u64)) {
                println!("Error in seeking to the end of the file: {}", err);
            }
            if let Err(err) = file_ref.write_all(&blank) {
                println!("Error in writing to the end of the file: {}", err);
            }

            let _ = file_ref.seek(SeekFrom::Start((buffer_len + remainder) as u64));
            let _ = file_ref.write_all(fs_base::HEADER_TAIL.as_bytes());
        }
    }
    fn write_block_remainder(
        &mut self,
        buffer: &mut &[u8],
        block_index: u32,
        file: Rc<RefCell<fs::File>>,
        from: usize,
    ) -> bool {
        if from.is_multiple_of(fs_base::BLOCK_CAPACITY) {
            return false;
        }
        let remainder = fs_base::BLOCK_CAPACITY - from;
        let to_be_written = &buffer[0..remainder];

        let mut file_ref = file.borrow_mut();
        let _ = file_ref.seek(SeekFrom::Start(
            (fs_base::HEADER_SIZE + from + (block_index as usize) * fs_base::BLOCK_CAPACITY) as u64,
        ));
        let _ = file_ref.write_all(to_be_written);

        *buffer = &buffer[remainder..];
        true
    }
}

impl Serde for BlockDevice {
    fn serialize(&self) -> Vec<u8> {
        let mut res = Vec::new();
        res.extend_from_slice(fs_base::BLOCK_DEVICE_PREAMBLE.as_bytes());
        res.extend_from_slice(&serde::ser_vec_u32(&self.free_blocks));
        res.extend_from_slice(&self.num_blocks.to_be_bytes());
        res
    }
    fn deserialize(buffer: &mut &[u8]) -> Result<Self, Error> {
        if !buffer.starts_with(fs_base::BLOCK_DEVICE_PREAMBLE.as_bytes()) {
            return Err(Error::InvalidPreamble(
                "Did not find the preamble for block device".to_string(),
            ));
        }
        *buffer = &buffer[fs_base::BLOCK_DEVICE_PREAMBLE.len()..];
        let free_blocks = serde::deser_vec_u32(buffer)?;
        let n_blocks = serde::deser_u32(buffer)?;

        Ok(BlockDevice::from(free_blocks, n_blocks))
    }
}
impl Default for BlockDevice {
    fn default() -> Self {
        Self::new()
    }
}
