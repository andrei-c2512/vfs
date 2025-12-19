use crate::fs_base::{Permissions, Error};
use crate::util::date_time::DateTime;
use crate::string_buffer::StringBuffer;
use crate::traits::{Directive, Serde};
use crate::inode::INode;
use crate::serde;

pub struct Directory{
    inode : INode,

    // holds indexes to the children in the node buffer
    children : Vec<u32>,
}

impl Serde for Directory{
    fn serialize(&self) -> Vec<u8>{
        let mut res = Vec::new();
        res.extend_from_slice(
            &self.inode.serialize()
        );
        res.extend_from_slice(
            &serde::ser_vec_u32(&self.children)
        );

        res
    }
    fn deserialize(buffer : &mut &[u8]) -> Result<Self, Error>{
        let node = INode::deserialize(buffer)?;
        let block_indices = serde::deser_vec_u32(buffer)?;
        
        Ok(Directory::from(node, block_indices))
    }

}

impl Directive for Directory {
    // returns none on valid string
    fn has_child(&self, str_buf : &StringBuffer, s : &String) -> bool{
        let str_id = str_buf.get(s);
        if let Ok(id) = str_id {
            return self.has_child_by_id(id);
        }

        false
    }
    fn has_child_by_id(&self, id : u32) -> bool{
        if self.children.contains(&id) == true {
            true
        }else{
            false
        }
    }
}

impl Directory{
    fn from(n : INode, children : Vec<u32>) -> Self{
        Self{inode : n, children : children}
    }

    pub fn add_child(&mut self, c : u32){
        self.children.push(c);
    }
}


