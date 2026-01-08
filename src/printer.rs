use crate::header::Node;
//use crate::directory::DirectoryData;
use crate::header::Header;
use crate::string_buffer::StringBuffer;
//use crate::file::File;

// --- REWRITE: Instead of this printer module, use the Display/Debug trait
pub fn print_header(header: &Header) {
    print_tree(0, &header.node_buffer, &header.str_buffer, 0);
}

pub fn tab(level: u32) -> String {
    if level == 0 {
        return String::new();
    }
    let mut res = String::with_capacity(level as usize);
    res.push_str("|-");
    for _ in 1..level - 1 {
        res.push_str("--");
    }
    if level != 1 {
        res.push_str("> ");
    }
    res
}

pub fn print_tree(node_id: u32, buffer: &Vec<Node>, str_buf: &StringBuffer, level: u32) {
    let n = &buffer[node_id as usize];
    match n {
        Node::Directory(dir) => {
            println!(
                "{}{} Created: {} ",
                tab(level),
                str_buf.string_list[dir.inode.name_id as usize],
                dir.inode.created_at
            );
            for entry_id in dir.children.iter() {
                print_tree(*entry_id, buffer, str_buf, level + 1);
            }
        }
        Node::File(file) => {
            println!(
                "{}{} Created: {} Last modified: {}",
                tab(level),
                str_buf.string_list[file.inode.name_id as usize],
                file.inode.created_at,
                file.inode.last_modified
            );
        }
    }
}

pub fn print_string_buffer(str_buf: &StringBuffer) {
    println!("String buffer: ");
    for item in str_buf.string_list.iter() {
        println!("\t{}", item);
    }
}
