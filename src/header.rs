use crate::string_buffer::StringBuffer;
use crate::file::File;
use crate::directory::Directory;
use crate::fs_base::Error;
use crate::traits::Directive;
use crate::util::string_helper;

use std::collections::HashMap;



pub enum Node{
    Directory(Directory),
    File(File),
}

impl Directive for Node{
     fn has_child(&self, str_buf : &StringBuffer, s : &String) -> bool{
         match self {
             Node::Directory(dir) => {
                 dir.has_child(str_buf, s)
             }
             Node::File(file) => {
                 file.has_child(str_buf, s)
             }
         }
    }
    fn has_child_by_id(&self, id : u32) -> bool{
         match self {
             Node::Directory(dir) => {
                 dir.has_child_by_id(id)
             }
             Node::File(file) => {
                 file.has_child_by_id(id)
             }
         }
    }
}

pub struct Header{
    node_buffer : Vec<Node>,
    root_map : HashMap<String, u32>,
    str_buffer : StringBuffer
}

impl Header{
    pub fn add_node(&mut self, path : &str, n : Node) -> Option<Error>{
        let parent_id = match self.navigate(path){
            Ok(id) => id,
            Err(err) => {return Some(err);}
        };
        let node_id = self.push_node(n);
        let mut parent_ref = &mut self.node_buffer[parent_id as usize];

        match &mut parent_ref{
            Node::Directory(dir) => {
                dir.add_child(node_id);
            }
            Node::File(_) => {
                return Some(Error::InvalidPath("Cannot add to a file to a file.".to_string()));
            }
        };
         

        None 
    }
    
    fn push_node(&mut self, n : Node) -> u32{
        let id = self.node_buffer.len();
        self.node_buffer.push(n);
        id as u32
    }
    fn navigate(&self, path : &str) -> Result<u32,Error> {
        let steps = string_helper::split_path(path);
        if steps.len() == 0 {
            return Err(Error::EmptyPath("Provided an empty path".to_string()));
        }
        let current_node = self.root_map[steps.get(0).unwrap()];
        for step in steps.iter().skip(1){
            let str_id = self.str_buffer.get(step)?;
            let node_ref = &self.node_buffer[current_node as usize];
            if node_ref.has_child_by_id(str_id) == false {
                return Err(Error::InvalidPath("Provided an invalid path at.".to_string()));
            }

        }
        // let next_node = self.node_buffer;
        Ok(current_node)
    }
}


