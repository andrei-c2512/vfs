use crate::header::{Header, Node};
use crate::fs_base::Error;
use crate::block_device::BlockDevice;
use crate::file::FileData;

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
            }
            Node::Directory(dir) => {
            }
        }
        None
    }
    pub fn append(&self, bytes : &[u8]) -> Option<Error> {

        None
    }
}
