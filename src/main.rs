pub mod util;
pub mod string_buffer;
pub mod errors;
pub mod serde;

use std::collections::HashMap;

use crate::util::date_time::DateTime;
use crate::util::string_helper;
use crate::string_buffer::StringBuffer;
use crate::errors::Error;


type Permissions = u16;

const READ : u16 = 1 << 0;
const WRITE : u16 = 1 << 1;

trait Directive{
    fn has_child(&self, str_buf : &StringBuffer, s : &String) -> bool;
    fn has_child_by_id(&self, id : u32) -> bool;
}


struct Directory{
    name_id : u32,
    permissions : Permissions,
    created_at : DateTime,
    last_modified : DateTime,

    children : Vec<u32>,
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


struct File{
    name_id : u32,
    permissions : Permissions,
    created_at : DateTime,
    last_modified : DateTime,

    block_indices : Vec<u32>,
}

impl Directive for File {
    fn has_child(&self, str_buf : &StringBuffer, s : &String) -> bool{
        false
    }
    fn has_child_by_id(&self, id : u32) -> bool{
        false
    }
}

enum Node{
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

struct FsHeader{
    node_buffer : Vec<Node>,
    root_map : HashMap<String, u32>,
    str_buffer : StringBuffer
}

impl FsHeader{
    fn add_dir(path : &str, dir : Directory) {
        let n = Node::Directory(dir);

    }
    fn add_file(file : File){

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



fn main() {
    println!("Hello, world!");
}
