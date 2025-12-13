use std::collections::HashMap;

pub mod util;

use crate::util::date_time::DateTime;

type Permissions = u16;

const READ : u16 = 1 << 0;
const WRITE : u16 = 1 << 1;

struct StringBuffer{
    name_map : HashMap<String, u32>,
    serialization_size : u32,
    next_index : u32,
}

impl StringBuffer{
    fn add(&mut self, name : &String) -> u32{
        self.name_map.insert(name.clone(), self.next_index);
        let copy = self.next_index;
        self.next_index += 1;
        return copy;
    }

    fn serialize(&self) -> Vec<u8>{

        let mut buffer = Vec::with_capacity(0);


        buffer
    }

    fn serialization_size(&self) -> u32{
    }
}

struct Directory{
    name_id : u32,
    children : Vec<u32>,
    permissions : Permissions,
    created_at : DateTime,
    last_modified : DateTime,
}

struct File{
    name_id : u32,
    block_indices : Vec<u32>,
    permissions : Permissions,
    created_at : DateTime,
    last_modified : DateTime,
}

enum Node{
    Directory(Directory),
    File(File),
}



struct FsHeader{
    tree_buffer : Vec<Node>,
}

impl FsHeader{
}


fn main() {
    println!("Hello, world!");
}
