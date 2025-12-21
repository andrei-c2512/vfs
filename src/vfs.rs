use crate::header::{Header,Node};
use crate::directory::Directory;
use crate::inode::INode;
use crate::file::File;
use crate::util::date_time::DateTime;
use crate::fs_base::Error;
use crate::string_buffer::StringBuffer;
use crate::printer;


pub struct Vfs{
    header : Header,
}

impl Vfs{
    pub fn open(&self, path : &str) {
        
    }
    pub fn create_dir(&mut self, path : &str) -> Option<Error>{
        let last_slash_ind = path.rfind('/').unwrap_or(0);
        println!("{}", last_slash_ind);
        let (last_directive, path_to_directive) = match last_slash_ind {
            0 => { (path, path) }
            _ => { (&path[last_slash_ind + 1..], &path[..last_slash_ind])}
        };

        let name_id = self.header.str_buffer.add(last_directive);
        let dir = Directory::from(
            INode::from(name_id, 0, DateTime::now(), DateTime::now(), 0), Vec::new()
        ); 

        return self.header.add_node(path_to_directive, name_id, Node::Directory(dir));
    }

    pub fn create(path : &str) -> Self{        
        Self{ header : Header::new() }
    }
    
    pub fn print(&self) {
        printer::print_header(&self.header);
        println!("{}", self.header.node_buffer.len());
        println!("{}", self.header.str_buffer.string_list.len());
    }
    fn init() {

    }
}
