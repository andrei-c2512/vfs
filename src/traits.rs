use crate::fs_base::Error;
use crate::string_buffer::StringBuffer;

pub trait Serde {
    fn serialize(&self) -> Vec<u8>;
    fn deserialize(buffer: &mut &[u8]) -> Result<Self, Error>
    where
        Self: Sized;
}

pub trait Directive {
    fn has_child(&self, str_buf: &StringBuffer, s: &str) -> bool;
    fn has_child_by_id(&self, id: u32) -> bool;
}
