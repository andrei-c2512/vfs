use crate::header::{Header, Node};
use crate::fs_base::Error;
use crate::block_device::BlockDevice;
//use crate::file::FileData;

use std::fs;
use std::rc::Rc;
use std::cell::RefCell;


pub struct File{
    file_id : u32,
    header : Rc<RefCell<Header>>,
    block_device : Rc<RefCell<BlockDevice>>,
    vfs_file : Rc<RefCell<fs::File>>,
}

impl File{
    pub fn from(file_id : u32, header : Rc<RefCell<Header>>, block_device : Rc<RefCell<BlockDevice>>, vfs_file : Rc<RefCell<fs::File>>) -> Self{
        Self { file_id : file_id, header : header, block_device : block_device , vfs_file : vfs_file }
    }
    pub fn write_all(&mut self, buffer : &[u8]) -> Option<Error>{
        let node = &mut self.header.borrow_mut().node_buffer[self.file_id as usize];        
        
        match node {
            Node::File(file_data) => {
                self.block_device.borrow_mut().write(buffer, &mut file_data.block_indices, self.vfs_file.clone());
                file_data.inode.size = buffer.len();
            }
            Node::Directory(_) => {
            }
        }
        None
    }
    pub fn append(&self, _bytes : &[u8]) -> Option<Error> {
        panic!("Called unimplemented function 'append'");
    }
    /* This endless match-pattern stuff I see is only and only because I don't store directories
     * and files in different lists. Bruh */
    pub fn read_to_string(&mut self, data : &mut String){
        let node = &mut self.header.borrow_mut().node_buffer[self.file_id as usize];        
        match node {
            Node::File(file_data) => {
                self.block_device.borrow_mut().read_to_string(data, &file_data.block_indices, file_data.inode.size, self.vfs_file.clone());
            }
            Node::Directory(_) => {}
        }
    }
    pub fn read(&mut self, data :&mut Vec<u8>) {
        let node = &mut self.header.borrow_mut().node_buffer[self.file_id as usize];        
        match node {
            Node::File(file_data) => {
                self.block_device.borrow_mut().read(data, &file_data.block_indices, file_data.inode.size, self.vfs_file.clone());
            }
            Node::Directory(_) => {}
        }
    }
}

pub struct Directory{
    /*
    dir_id : u32,
    header : Rc<RefCell<Header>>,
    block_device : Rc<RefCell<BlockDevice>>,
    vfs_file : Rc<RefCell<fs::File>>,
    */
}
impl Directory {
    pub fn new() -> Self{
        Self{}
    }
}


