pub mod util;
pub mod string_buffer;
pub mod directory;
pub mod fs_base;
pub mod file;
pub mod traits;
pub mod header;
pub mod inode;
pub mod serde;
pub mod vfs;
pub mod printer;

use crate::vfs::Vfs;

fn main() {
    let mut vfs = Vfs::create("test.vfs");
    let paths = [ "@/etc", "@/etc/conf", "@/etc/tmp", "@/etc/tmp/p2", "@/etc/tmp/p3", "@/etc/work" ];
    for path in paths {
        match vfs.create_dir(path){
            Some(err) => {
                println!("Error: {}" , err);
            }
            None => {
            }
        }
    }
    vfs.print(); 
}
