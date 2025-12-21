use crate::fs_base::Error;
use crate::traits::{Directive, Serde};
use crate::string_buffer::StringBuffer;
use crate::inode::INode;
use crate::serde;

pub struct File{
    pub inode : INode,

    pub block_indices : Vec<u32>,
}

impl File{
    fn from(n : INode, blocks : Vec<u32>) -> Self{
        Self{inode : n, block_indices : blocks}
    }
}

impl Serde for File{
    fn serialize(&self) -> Vec<u8>{
        let mut res = Vec::new(); 
        res.extend_from_slice(
            &self.inode.serialize()
        );
        res.extend_from_slice(
            &serde::ser_vec_u32(&self.block_indices)
        );

        res
    }
    fn deserialize(buffer : &mut &[u8]) -> Result<Self, Error> {
        let node = INode::deserialize(buffer)?;
        let block_indices = serde::deser_vec_u32(buffer)?;
        
        Ok(File::from(node, block_indices))
    }

}

impl Directive for File {
    fn has_child(&self, _str_buf : &StringBuffer, _s : &String) -> bool{
        false
    }
    fn has_child_by_id(&self, _id : u32) -> bool{
        false
    }
}



