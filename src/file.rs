use crate::fs_base::{BLOCK_CAPACITY, Error};
use crate::inode::INode;
use crate::serde;
use crate::string_buffer::StringBuffer;
use crate::traits::{Directive, Serde};

pub struct FileData {
    pub inode: INode,

    pub block_indices: Vec<u32>,
}

impl FileData {
    pub fn from(n: INode, blocks: Vec<u32>) -> Self {
        Self {
            inode: n,
            block_indices: blocks,
        }
    }
    pub fn capacity(&self) -> usize {
        // may change in the future
        self.block_indices.len() * BLOCK_CAPACITY
    }
}

impl Serde for FileData {
    fn serialize(&self) -> Vec<u8> {
        let mut res = Vec::new();
        res.extend_from_slice(&self.inode.serialize());
        res.extend_from_slice(&serde::ser_vec_u32(&self.block_indices));

        res
    }
    fn deserialize(buffer: &mut &[u8]) -> Result<Self, Error> {
        let node = INode::deserialize(buffer)?;
        let block_indices = serde::deser_vec_u32(buffer)?;

        Ok(FileData::from(node, block_indices))
    }
}

impl Directive for FileData {
    fn has_child(&self, _str_buf: &StringBuffer, _s: &str) -> bool {
        false
    }
    fn has_child_by_id(&self, _id: u32) -> bool {
        false
    }
}
