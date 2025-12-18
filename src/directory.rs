use crate::fs_base::Permissions;
use crate::util::date_time::DateTime;
use crate::string_buffer::StringBuffer;
use crate::traits::Directive;

pub struct Directory{
    name_id : u32,
    permissions : Permissions,
    created_at : DateTime,
    last_modified : DateTime,
    size : usize,

    // holds indexes to the children in the node buffer
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

impl Directory{
    pub fn add_child(&mut self, c : u32){
        self.children.push(c);
    }
}


