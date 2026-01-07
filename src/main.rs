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
use crate::fs_base::Error;
use crate::util::string_helper;

use std::fs;

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
        if let Err(err) = vfs.create_dir(path){
            println!("Error: {}" , err);
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
                let buf = [b'$';100];
                file.write_all(&buf);
                let mut data = String::new();
                file.read_to_string(&mut data);
                println!("Printing file contents:\n{}", data);
            }
        }
    }
    vfs.print(); 
}

fn read_test(path : &str) {
    let vfs = match Vfs::open(path){
        Ok(vfs) => { vfs}
        Err(err) => { println!("{}", err); return; }
    };
    vfs.print(); 
}

fn lab_example_test() -> Result<(), Error>{
    let mut vfs = Vfs::new("realfile.vfs")?;

    vfs.create_dir("@/rs")?;
    {
        let mut f1 = vfs.create("@/rs/abc.txt")?;
        let mut f2 = vfs.create("@/rs/def.txt")?;

        f1.write_all(b"hello");
        f2.write_all(b"world");
    }

    let mut data = String::new();
    for entry in vfs.read_dir("@/rs")? {
        data.clear();

        let mut file = vfs.open_entry(entry)?;
        file.read_to_string(&mut data);

        print!("{}", data);
    }
    println!();
    Ok(())
}

fn size_test_3mb() -> Result<(), Error> {
    let mut vfs = Vfs::new("medium.vfs")?;
    let vfs_img_file = "@/img.jpeg";
    let input_os_file = "output/background.jpeg";
    vfs.copy_into_vfs("res/background.jpeg", vfs_img_file)?;

    let mut file = match fs::OpenOptions::new()
        .truncate(true)
        .open(input_os_file) {
        Ok(file) => {file }
        Err(err) => { return Err(
                Error::FileOps(string_helper::fmt_file_error(&err.to_string(), input_os_file))); 
        } 
    };

    vfs.copy_from_vfs(vfs_img_file, &mut file)?;
    Ok(())
}

fn size_test_12mb() -> Result<(), Error> {
    let mut vfs = Vfs::new("large.vfs")?;
    let vfs_img_file = "@/img.bmp";
    let input_os_file = "output/background.bmp";
    vfs.copy_into_vfs("res/background.bmp", vfs_img_file)?;

    let mut file = match fs::OpenOptions::new()
        .truncate(true)
        .open(input_os_file) {
        Ok(file) => {file }
        Err(err) => { return Err(
                Error::FileOps(string_helper::fmt_file_error(&err.to_string(), input_os_file))); 
        } 
    };

    vfs.copy_from_vfs(vfs_img_file, &mut file)?;
    Ok(())
}

/* 
 * This tests having multiple small files on the system
 */
fn test_2() -> Result<(), Error> {
    let mut vfs = Vfs::new("test2.vfs")?;
    let vfs_files = [ "@/rs/abc.txt", "@/rs/def.txt"] ;
    let os_files = [ "res/test1.txt", "res/test2.txt"] ;


    //let vfs_files = [ "@/rs/abc.txt"] ;
    //let os_files = [ "res/test1.txt"] ;
    vfs.create_dir("@/rs")?;
    {
        for i in 0..vfs_files.len() {
            vfs.copy_into_vfs(os_files[i], vfs_files[i])?;
        }
    }
    
    for i in 0..vfs_files.len() {
        let mut f = vfs.open_file(vfs_files[i])?;
        let mut data = String::new();
        f.read_to_string(&mut data);
        println!("Result of reading file '{}': {:?}", vfs_files[i], data);
    }


    Ok(()) 
}

/* 
 * This tests having multiple big files on the system
 */
fn test_3() -> Result<(), Error> {
    let mut vfs = Vfs::new("complex.vfs")?;
    let paths = [ "@/etc", "@/etc/conf", "@/etc/tmp", "@/etc/tmp/p2", "@/etc/tmp/p3", "@/etc/work" ];
    for path in paths {
        if let Err(err) = vfs.create_dir(path){
            println!("Error: {}" , err);
        }
    }
    
    vfs.copy_into_vfs("res/background.jpeg", "@/img1.jpeg")?; 
    vfs.copy_into_vfs("res/background.jpeg", "@/img2.jpeg")?;

    let mut file = match fs::OpenOptions::new()
        .truncate(true)
        .open("output/background.jpeg") {
        Ok(file) => {file }
        Err(err) => { return Err(
                Error::FileOps(string_helper::fmt_file_error(&err.to_string(), "output/background.jpeg"))); 
        } 
    };

    vfs.copy_from_vfs("@/img2.jpeg", &mut file)?;
    Ok(())
}

/* 
 * This tests having multiple big files on the system while tryinig to achieve a "dispersed" under
 * the hood representation of some files, to see how the system behaves
 */
fn test_4() -> Result<(), Error>{
    let mut vfs = Vfs::new("complex.vfs")?;
    let paths = [ "@/etc", "@/etc/conf", "@/etc/tmp", "@/etc/tmp/p2", "@/etc/tmp/p3", "@/etc/work" ];
    for path in paths {
        if let Err(err) = vfs.create_dir(path){
            println!("Error: {}" , err);
        }
    }
    
    vfs.copy_into_vfs("res/background.jpeg", "@/img1.jpeg")?; 
    vfs.copy_into_vfs("res/background.bmp", "@/img2.bmp")?;
    vfs.copy_into_vfs("res/background.jpeg", "@/img3.bmp")?;
    vfs.copy_into_vfs("res/background.jpeg", "@/img4.jpeg")?; 
    
    // overwriting the file, making it be split in 2 (because img4 is in the middle)
    vfs.copy_into_vfs("res/background.bmp", "@/img3.bmp")?;

    let mut file = match fs::OpenOptions::new()
        .truncate(true)
        .open("output/background_test4.jpeg") {
        Ok(file) => {file }
        Err(err) => { return Err(
                Error::FileOps(string_helper::fmt_file_error(&err.to_string(), "output/background_test4.jpeg"))); 
        } 
    };

    //vfs.copy_from_vfs("@/img3.bmp", &mut file)?;

    vfs.copy_from_vfs("@/img3.bmp", &mut file)?;
    Ok(())
}


fn main() -> Result<(), Error>{
    test::directory_serde();
    test::string_buffer_serde();
    create_test();
    read_test("test.vfs"); 
    lab_example_test()?;
    size_test_3mb()?;
    size_test_12mb()?;
    test_2()?;
    test_3()?;
    test_4()?;

    Ok(())
}
