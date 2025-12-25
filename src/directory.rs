use crate::fs_base::Error;
use crate::string_buffer::StringBuffer;
use crate::traits::{Directive, Serde};
use crate::inode::INode;
use crate::serde;
use std::collections::HashMap;

#[derive(PartialEq, Eq)]
pub struct DirectoryData{
    pub inode : INode,

    pub children : Vec<u32>,
    // this is held here for convenience purposes
    pub name_child_map : HashMap<u32, u32>,
}

impl Serde for DirectoryData{
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
        println!("{:?}", buffer);
        println!("Size before deserializng inode: {}", buffer.len());

        let node = INode::deserialize(buffer)?;
        println!("{:?}", buffer);
        println!("Size before deserializng inode: {}", buffer.len());
        let block_indices = serde::deser_vec_u32(buffer)?; 
        println!("{:?}", buffer);
        println!("Size after deserializng inode: {}", buffer.len());

        Ok(DirectoryData::from(node, block_indices))
    }

}

impl Directive for DirectoryData{
    // returns none on valid string
    fn has_child(&self, str_buf : &StringBuffer, s : &String) -> bool{
        let str_id = str_buf.get(s);
        if let Ok(id) = str_id {
            return self.has_child_by_id(id);
        }

        false
    }
    fn has_child_by_id(&self, name_id : u32) -> bool{
        if self.name_child_map.contains_key(&name_id) == true {
            true
        }else{
            false
        }
    }
}

impl DirectoryData{
    pub fn new() -> Self{
        Self{inode : INode::new(), children : Vec::new(), name_child_map : HashMap::new() }
    }
    pub fn from(n : INode, children : Vec<u32>) -> Self{
        Self{inode : n, children : children, name_child_map : HashMap::new()}
    }

    pub fn add_child(&mut self, name_id : u32, node_id : u32){
        self.name_child_map.insert(name_id, node_id);
        self.children.push(node_id);
    }
}


