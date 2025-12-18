use crate::string_buffer::StringBuffer;

pub trait Directive{
    fn has_child(&self, str_buf : &StringBuffer, s : &String) -> bool;
    fn has_child_by_id(&self, id : u32) -> bool;
}

