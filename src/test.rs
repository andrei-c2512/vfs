use crate::directory::DirectoryData;
use crate::inode::INode;
use crate::util::date_time::DateTime;
use crate::traits::Serde;
use crate::string_buffer::StringBuffer;
use crate::util::string_helper;
use crate::printer;
use crate::fs_base::Error;
use crate::vfs::Vfs;

use std::thread;
use std::time::Duration;
use std::fs;

/*
static RED : i32= 41;
static GREEN : i32= 42;
static WHITE : i32 = 47;

fn println_colored(text : &str, color : i32){
    println!("\\u001b[{}m{}\\u001b[47m", color, text);
}
*/
fn write_test_header(num : i32, details : &str) {
    println!("--- TEST {}: {}", num, details);
    println!("...\n\n");
}


fn passed(){
    //println_colored("TEST PASSED", GREEN);
    println!("--- TEST PASSED\n\n");
}
pub fn directory_serde(){
    write_test_header(1, "Testing serialization and deserialization of the directory data");
    let children = vec![10, 23, 11, 14];
    let dir = DirectoryData::from(
        INode::from(3, 10, DateTime::now(), DateTime::now(), 100), children
    );

    let data = dir.serialize();
    
    match DirectoryData::deserialize(&mut data.as_slice()){
        Ok(dir2) => {
            if dir != dir2 {
                println!("Serde test for directory failed! Reason: unequal results after serialization and deserialization");
                return;
            }
        }
        Err(err) => {
            println!("Serde test for directory failed! Reason: {}", err);
            return;
        }
    }
    passed();
}

pub fn string_buffer_serde(){
    write_test_header(2, "Testing serialization and deserialization of the directory data");
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
            return;
        }
    }

    passed();
}

fn create_test(){
    write_test_header(3, "Testing the creation of a simple directory tree");
    let mut vfs = match Vfs::new("test.vfs"){
        Ok(vfs) => { vfs }
        Err(err) => {
            println!("{}", err);
            return;
        }
    };
    let paths = [ "etc", "etc/conf", "etc/tmp", "etc/tmp/p2", "etc/tmp/p3", "etc/work" ];
    for path in paths {
        if let Err(err) = vfs.create_dir(path){
            println!("Error: {}" , err);
        }
    }

    //let file_paths = [ "etc/file.txt"];
    let file_paths = [ "etc/file.txt", "etc/tmp/file2.txt", "etc/work/file.txt" ];

    for path in file_paths {
        let mut res = vfs.create(path);
        match &mut res{
            Err(err) => {
                println!("Error: {}" , err);
                return;
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
    passed();
}

fn read_test(path : &str) {
    write_test_header(4, "Testing the reading of a simple directory tree");
    let vfs = match Vfs::open(path){
        Ok(vfs) => { vfs}
        Err(err) => { println!("{}", err); return; }
    };
    vfs.print(); 
    passed();
}

fn lab_example_test() -> Result<(), Error>{
    write_test_header(5, "Testing API example from lab");
    let mut vfs = Vfs::new("realfile.vfs")?;

    vfs.create_dir("rs")?;
    {
        let mut f1 = vfs.create("rs/abc.txt")?;
        let mut f2 = vfs.create("rs/def.txt")?;

        f1.write_all(b"hello");
        f2.write_all(b"world");
    }

    let mut data = String::new();
    for entry in vfs.read_dir("rs")? {
        data.clear();

        let mut file = vfs.open_entry(entry)?;
        file.read_to_string(&mut data);

        println!("{}", data);
    }
    passed();
    vfs.print();
    Ok(())
}

fn size_test_3mb() -> Result<(), Error> {
    write_test_header(6, "Testing moving a 3MB file on the vfs");
    let mut vfs = Vfs::new("medium.vfs")?;
    let vfs_img_file = "img.jpeg";
    let input_os_file = "output/background.jpeg";
    vfs.copy_into_vfs("res/background.jpeg", vfs_img_file)?;

    let mut file = match fs::OpenOptions::new()
        .truncate(true)
        .write(true)
        .read(true)
        .open(input_os_file) {
        Ok(file) => {file }
        Err(err) => { return Err(
                Error::FileOps(string_helper::fmt_file_error(&err.to_string(), input_os_file))); 
        } 
    };

    vfs.copy_from_vfs(vfs_img_file, &mut file)?;
    passed();
    Ok(())
}

fn size_test_12mb() -> Result<(), Error> {
    write_test_header(7, "Testing moving a 12MB file on the vfs");
    let mut vfs = Vfs::new("large.vfs")?;
    let vfs_img_file = "img.bmp";
    let input_os_file = "output/background.bmp";
    vfs.copy_into_vfs("res/background.bmp", vfs_img_file)?;

    let mut file = match fs::OpenOptions::new()
        .truncate(true)
        .read(true)
        .write(true)
        .open(input_os_file) {
        Ok(file) => {file }
        Err(err) => { return Err(
                Error::FileOps(string_helper::fmt_file_error(&err.to_string(), input_os_file))); 
        } 
    };

    vfs.copy_from_vfs(vfs_img_file, &mut file)?;

    passed();
    Ok(())
}

/* 
 * This tests having multiple small files on the system
 */
fn test_2() -> Result<(), Error> {
    write_test_header(8, "Testing having multiple directories AND small files");
    let mut vfs = Vfs::new("test2.vfs")?;
    let vfs_files = [ "rs/abc.txt", "rs/def.txt"] ;
    let os_files = [ "res/test1.txt", "res/test2.txt"] ;


    //let vfs_files = [ "rs/abc.txt"] ;
    //let os_files = [ "res/test1.txt"] ;
    vfs.create_dir("rs")?;
    {
        for i in 0..vfs_files.len() {
            vfs.copy_into_vfs(os_files[i], vfs_files[i])?;
        }
    }
    
    for path in vfs_files.iter() {
        let mut f = vfs.open_file(path)?;
        let mut data = String::new();
        f.read_to_string(&mut data);
        println!("Result of reading file '{}': {:?}", path, data);
    }

    passed();
    Ok(()) 
}

/* 
 * This tests having multiple big files on the system
 */
fn test_3() -> Result<(), Error> {
    write_test_header(9, "Testing having multiple directories AND big files");
    let mut vfs = Vfs::new("complex.vfs")?;
    let paths = [ "etc", "etc/conf", "etc/tmp", "etc/tmp/p2", "etc/tmp/p3", "etc/work" ];
    for path in paths {
        if let Err(err) = vfs.create_dir(path){
            println!("Error: {}" , err);
        }
    }
    
    vfs.copy_into_vfs("res/background.jpeg", "img1.jpeg")?; 
    vfs.copy_into_vfs("res/background.jpeg", "img2.jpeg")?;

    let mut file = match fs::OpenOptions::new()
        .truncate(true)
        .write(true)
        .read(true)
        .open("output/background.jpeg") {
        Ok(file) => {file }
        Err(err) => { return Err(
                Error::FileOps(string_helper::fmt_file_error(&err.to_string(), "output/background.jpeg"))); 
        } 
    };

    vfs.copy_from_vfs("img2.jpeg", &mut file)?;

    passed();
    Ok(())
}

/* 
 * This tests having multiple big files on the system while tryinig to achieve a "dispersed" under
 * the hood representation of some files, to see how the system behaves
 */
fn _test_4() -> Result<(), Error>{
    write_test_header(10, "Testing having multiple directories AND big files + overwriting a file in the middle of 
                      the block device, to observe fragmented file correcteness");
    let mut vfs = Vfs::new("complex.vfs")?;
    let paths = [ "etc", "etc/conf", "etc/tmp", "etc/tmp/p2", "etc/tmp/p3", "etc/work" ];
    for path in paths {
        if let Err(err) = vfs.create_dir(path){
            println!("Error: {}" , err);
        }
    }
    
    vfs.copy_into_vfs("res/background.jpeg", "img1.jpeg")?; 
    vfs.copy_into_vfs("res/background.bmp", "img2.bmp")?;
    vfs.copy_into_vfs("res/background.jpeg", "img3.bmp")?;
    vfs.copy_into_vfs("res/background.jpeg", "img4.jpeg")?; 
    
    // overwriting the file, making it be split in 2 (because img4 is in the middle)
    thread::sleep(Duration::from_secs(3));
    vfs.copy_into_vfs("res/background.bmp", "img3.bmp")?;

    let mut file = match fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open("output/background_test4.bmp") {
        Ok(file) => {file }
        Err(err) => { return Err(
                Error::FileOps(string_helper::fmt_file_error(&err.to_string(), "output/background_test4.bmp"))); 
        } 
    };

    //vfs.copy_from_vfs("img3.bmp", &mut file)?;

    vfs.copy_from_vfs("img3.bmp", &mut file)?;
    vfs.print();    

    passed();
    Ok(())
}

pub fn run_all() -> Result<(), Error>{
    directory_serde();
    string_buffer_serde();
    create_test();
    read_test("test.vfs"); 
    lab_example_test()?;
    size_test_3mb()?;
    size_test_12mb()?;
    test_2()?;
    test_3()?;
    _test_4()?;

    Ok(())
}
