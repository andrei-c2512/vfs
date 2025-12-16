pub mod util;
pub mod string_buffer;
pub mod errors;
pub mod serde;

use crate::util::date_time::DateTime;
use crate::string_buffer::StringBuffer;


type Permissions = u16;

const READ : u16 = 1 << 0;
const WRITE : u16 = 1 << 1;


struct Directory{
    name_id : u32,
    permissions : Permissions,
    created_at : DateTime,
    last_modified : DateTime,

    children : Vec<u32>,
}

impl Directory{
}

struct File{
    name_id : u32,
    permissions : Permissions,
    created_at : DateTime,
    last_modified : DateTime,

    block_indices : Vec<u32>,
}

enum Node{
    Directory(Directory),
    File(File),
}

struct FsHeader{
    tree_buffer : Vec<Node>,
    str_header : StringBuffer
}

impl FsHeader{
    fn add_dir(path : &str, dir : Directory) {
        let n = Node::Directory(dir);
    }
    fn add_file(file : File){

    }
}


fn main() {
    println!("Hello, world!");
}
