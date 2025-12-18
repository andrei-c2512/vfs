use crate::fs_base::Permissions;
use crate::util::date_time::DateTime;
use crate::traits::Directive;
use crate::string_buffer::StringBuffer;

pub struct File{
    name_id : u32,
    permissions : Permissions,
    created_at : DateTime,
    last_modified : DateTime,
    size : usize,

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


