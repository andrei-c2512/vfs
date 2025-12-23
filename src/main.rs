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
pub mod test;

use crate::vfs::Vfs;
use crate::directory::Directory;

fn create_test(){
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

fn read_test() {
    let vfs = match Vfs::open("test.vfs"){
        Ok(vfs) => { vfs}
        Err(err) => { println!("{}", err); return; }
    };
    vfs.print(); 
}

fn advance(sl : &mut &[u8]) {
    *sl = &sl[2..];
}

fn main() {
    //test::directory_serde();
    //test::string_buffer_serde();
    read_test();
    //create_test();
    
}
