use crate::serde;
use crate::directory::DirectoryData;
use crate::inode::INode;
use crate::util::date_time::DateTime;
use crate::traits::Serde;
use crate::string_buffer::StringBuffer;
use crate::printer;

use std::collections::HashMap;

pub fn directory_serde(){
    let children = vec![10, 23, 11, 14];
    let mut dir = DirectoryData::from(
        INode::from(3, 10, DateTime::now(), DateTime::now(), 100), children
    );

    let data = dir.serialize();
    
    match DirectoryData::deserialize(&mut data.as_slice()){
        Ok(dir2) => {
            if dir != dir2 {
                println!("Serde test for directory failed! Reason: unequal results after serialization and deserialization");
            }
        }
        Err(err) => {
            println!("Serde test for directory failed! Reason: {}", err);
        }
    }
}

pub fn string_buffer_serde(){
    let mut str_buf = StringBuffer::new();
    str_buf.add("etc");
    str_buf.add("bin");
    str_buf.add("run");
    str_buf.add("conf");
    str_buf.add("home");

    let data = str_buf.serialize();

    match StringBuffer::deserialize(&mut data.as_slice()){
        Ok(str_buf2) => {
            if str_buf != str_buf2 {
                println!("Serde test for string buffer failed! Reason: unequal results after serialization and deserialization");

                printer::print_string_buffer(&str_buf);
                printer::print_string_buffer(&str_buf2);
            }
        }
        Err(err) => {
            println!("Serde test for string buffer failed! Reason: {}", err);
        }
    }
}
