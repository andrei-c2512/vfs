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
pub mod ops;
pub mod block_device;

use crate::vfs::Vfs;

fn create_test(){
    let mut vfs = match Vfs::new("test.vfs"){
        Ok(vfs) => { vfs }
        Err(err) => {
            println!("{}", err);
            return;
        }
    };
    let paths = [ "@/etc", "@/etc/conf", "@/etc/tmp", "@/etc/tmp/p2", "@/etc/tmp/p3", "@/etc/work" ];
    for path in paths {
        match vfs.create_dir(path){
            Err(err) => {
                println!("Error: {}" , err);
            }
            Ok(_) => {
            }
        }
    }

    //let file_paths = [ "@/etc/file.txt"];
    let file_paths = [ "@/etc/file.txt", "@/etc/tmp/file2.txt", "@/etc/work/file.txt" ];

    for path in file_paths {
        let mut res = vfs.create(path);
        match &mut res{
            Err(err) => {
                println!("Error: {}" , err);
            }
            Ok(file)=> {
                let buf = ['$' as u8;100];
                file.write_all(&buf);
                let mut data = String::new();
                file.read_to_string(&mut data);
                println!("Printing file contents:\n{}", data);
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


fn main() {
    //test::directory_serde();
    //test::string_buffer_serde();
    create_test();
    read_test(); 
}
