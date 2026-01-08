use crate::directory::DirectoryData;
use crate::file::FileData;
use crate::fs_base::Error;
use crate::serde;
use crate::string_buffer::StringBuffer;
use crate::traits::{Directive, Serde};
use crate::util::string_helper;

//use std::collections::HashMap;

pub enum Node {
    Directory(DirectoryData),
    File(FileData),
}

impl Directive for Node {
    fn has_child(&self, str_buf: &StringBuffer, s: &str) -> bool {
        match self {
            Node::Directory(dir) => dir.has_child(str_buf, s),
            Node::File(file) => file.has_child(str_buf, s),
        }
    }
    fn has_child_by_id(&self, id: u32) -> bool {
        match self {
            Node::Directory(dir) => dir.has_child_by_id(id),
            Node::File(file) => file.has_child_by_id(id),
        }
    }
}

// --- REWRITE: Use enums instead of constants
impl Serde for Node {
    fn serialize(&self) -> Vec<u8> {
        let mut res = Vec::new();
        match self {
            Node::Directory(dir) => {
                res.push(0);
                res.extend_from_slice(&dir.serialize());
            }
            Node::File(file) => {
                res.push(1);
                res.extend_from_slice(&file.serialize());
            }
        }
        res
    }
    fn deserialize(buffer: &mut &[u8]) -> Result<Self, Error> {
        let node_type = serde::deser_u8(buffer)?;
        match node_type {
            0 => {
                //println!("Deserialising directory...");
                let dir = DirectoryData::deserialize(buffer)?;
                Ok(Node::Directory(dir))
            }
            1 => {
                let file = FileData::deserialize(buffer)?;
                Ok(Node::File(file))
            }
            _ => Err(Error::BadDeser("Unrecognized node type".to_string())),
        }
    }
}

fn serialize_node_list(list: &[Node]) -> Vec<u8> {
    let mut res = Vec::new();
    res.extend_from_slice(&(list.len() as u32).to_be_bytes());

    for n in list.iter() {
        res.extend_from_slice(&n.serialize());
    }
    res
}

fn deserialize_node_list(buffer: &mut &[u8]) -> Result<Vec<Node>, Error> {
    //println!("Deserialising node list...");
    let capacity = serde::deser_u32(buffer)?;
    //println!("Capacity: {}", capacity);
    let mut res = Vec::with_capacity(capacity as usize);

    for _ in 0..capacity {
        let node = Node::deserialize(buffer)?;
        res.push(node);
    }

    //println!("Deserialization end for node list");
    Ok(res)
}

pub struct Header {
    pub node_buffer: Vec<Node>,
    pub str_buffer: StringBuffer,
}

impl Header {
    pub fn new() -> Self {
        let node_buffer = vec![Node::Directory(DirectoryData::new())];
        let mut str_buffer = StringBuffer::new();
        str_buffer.add("@");

        Self {
            node_buffer,
            str_buffer,
        }
    }
    pub fn from(node_buffer: Vec<Node>, str_buffer: StringBuffer) -> Self {
        Self {
            node_buffer,
            str_buffer,
        }
    }
    // returns the ID of the newly added node
    pub fn add_node(&mut self, path: &str, name_id: u32, n: Node) -> Result<u32, Error> {
        let parent_id = match self.navigate(path) {
            Ok(id) => id,
            Err(err) => {
                return Err(err);
            }
        };
        let node_id = self.push_node(n);
        let mut parent_ref = &mut self.node_buffer[parent_id as usize];

        match &mut parent_ref {
            Node::Directory(dir) => {
                //println!("Added child to path '{}'", path);
                dir.add_child(name_id, node_id);
            }
            Node::File(_) => {
                return Err(Error::InvalidPath(
                    "Cannot add to a file a file.".to_string(),
                ));
            }
        };
        Ok(node_id)
    }
    pub fn get_file_capacity(&self, file_id: u32) -> Result<usize, Error> {
        match &self.node_buffer[file_id as usize] {
            Node::File(file) => Ok(file.capacity()),
            _ => Err(Error::Unreachable(
                "Reached unreachable code. Should not call get_file_capacity on a directory"
                    .to_string(),
            )),
        }
    }
    /*
    pub fn try_to_get_entries(&self, path : &str) -> Result<Vec<u32>, Error>{
        let node_id = self.navigate(path)?;
        let node = &self.node_buffer[node_id as usize];

        match node {
        };
    }
    */
    fn push_node(&mut self, n: Node) -> u32 {
        let id = self.node_buffer.len();
        self.node_buffer.push(n);
        id as u32
    }
    pub fn navigate(&self, path: &str) -> Result<u32, Error> {
        // println!("{}", path);
        let steps = string_helper::split_path(path);
        //println!("{:?}", steps);
        if steps.is_empty() {
            return Err(Error::EmptyPath("Provided an empty path".to_string()));
        }

        let mut current_node = 0;

        for step in steps.iter().skip(1) {
            // --- REWRITE: I don't like the logic here. I do redundant checks
            let str_id = self.str_buffer.get(step)?;
            let node_ref = &self.node_buffer[current_node as usize];

            if !node_ref.has_child_by_id(str_id) {
                return Err(Error::InvalidPath("Provided an invalid path.".to_string()));
            }
            match node_ref {
                Node::Directory(dir) => {
                    current_node = dir.name_child_map[&str_id];
                }
                _ => {
                    return Err(Error::Unreachable("Unreachable code".to_string()));
                }
            }
        }
        // let next_node = self.node_buffer;
        //println!("Navigated to node: {}", current_node);
        Ok(current_node)
    }
}

impl Serde for Header {
    fn serialize(&self) -> Vec<u8> {
        let mut res = Vec::new();

        // we deduce the roots at runtime
        res.extend_from_slice(&serialize_node_list(&self.node_buffer));
        res.extend_from_slice(&self.str_buffer.serialize());

        res
    }
    fn deserialize(buffer: &mut &[u8]) -> Result<Self, Error> {
        //println!("{:?}", buffer);
        let list = deserialize_node_list(buffer)?;
        //println!("{:?}", buffer);
        let str_buf = StringBuffer::deserialize(buffer)?;

        Ok(Header::from(list, str_buf))
    }
}

impl Default for Header {
    fn default() -> Self {
        Self::new()
    }
}
