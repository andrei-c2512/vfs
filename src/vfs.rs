use crate::header::{Header,Node};
use crate::directory::DirectoryData;
use crate::file;
use crate::inode::INode;
use crate::util::date_time::DateTime;
//use crate::string_buffer::StringBuffer;
use crate::printer;
use crate::traits::Serde;
use crate::block_device::BlockDevice;
use crate::ops;
use crate::fs_base;
use crate::fs_base::Error;

use std::fs;
use std::io::{Seek, Write,SeekFrom};
use std::cell::RefCell;
use std::rc::Rc;


pub struct Vfs{
    header : Rc<RefCell<Header>>, 
    block_device : Rc<RefCell<BlockDevice>>,
    file : Rc<RefCell<fs::File>>,
}

impl Vfs{
    pub fn new(path : &str) -> Result<Self,fs_base::Error>{        
        let file = Self::create_file(path)?;
        let shared_file = Rc::new(RefCell::new(file));

        Ok(
            Self{ 
                header : Rc::new(RefCell::new(Header::new())), 
                block_device : Rc::new(RefCell::new(BlockDevice::new())),
                file : shared_file
            }
        )
    }
    pub fn open(path : &str) -> Result<Self,Error>{
        // ell biggo letto listo
        // --- REWRITE: create/find a "make_shared" equivalent
        let data = std::fs::read(path).unwrap();

        let slice =&mut data.as_slice();
        let header = Header::deserialize(slice)?;
        let shared_header = Rc::new(RefCell::new(header));

        let block_device = BlockDevice::deserialize(slice)?;
        let shared_bd = Rc::new(RefCell::new(block_device));

        let file = Self::open_file(path)?;
        let shared_file = Rc::new(RefCell::new(file));


        Ok(Self{header : shared_header, block_device : shared_bd, file : shared_file})
    }
    pub fn create_dir(&mut self, path : &str) -> Result<ops::Directory, Error>{
        let (last_directive, path_to_directive) = self.split_from_cwd(path);
        
        let dir_id = {
            let mut header_ref = self.header.borrow_mut();
            let name_id = header_ref.str_buffer.add(last_directive);
            let dir = DirectoryData::from(
                INode::from(name_id, 0, DateTime::now(), DateTime::now(), 0), Vec::new()
            ); 

            let dir_id = match header_ref.add_node(path_to_directive, name_id, Node::Directory(dir)) {
                Err(err) => { return Err(err); }
                Ok(dir_id) => { dir_id }
            };
            dir_id
        };
        self.update_file();

        Ok(ops::Directory::new())
    }
    pub fn create(&mut self, path : &str) -> Result<ops::File, Error> { 
        let (last_directive, path_to_directive) = self.split_from_cwd(path);
        let file_id = {
            let mut header_ref = self.header.borrow_mut();
            let name_id = header_ref.str_buffer.add(last_directive);
            let file : file::FileData = file::FileData::from(
                INode::from(name_id, 0, DateTime::now(), DateTime::now(), 0), Vec::new()
            ); 

            let file_id = match header_ref.add_node(path_to_directive, name_id, Node::File(file)) {
                Err(err) => { return Err(err); }
                Ok(file_id) => { file_id }
            };
            file_id
        }; 
        self.update_file();

        Ok(
            ops::File::from(file_id, self.header.clone(), self.block_device.clone(), self.file.clone())
        )
    }
    
    pub fn print(&self) {
        let header = self.header.borrow();
        printer::print_header(&header);
        println!("{}", header.node_buffer.len());
        println!("{}", header.str_buffer.string_list.len());
    }
    // dunno a good name for this
    fn split_from_cwd<'a>(&mut self, path : &'a str) -> (&'a str, &'a str) {
        let last_slash_ind = path.rfind('/').unwrap_or(0);
        println!("{}", last_slash_ind);
        match last_slash_ind {
            0 => { return (path, path); }
            _ => { return (&path[last_slash_ind + 1..], &path[..last_slash_ind]);}
        };
    }
    fn create_file(path : &str) -> Result<fs::File, Error> {
        let res = fs::OpenOptions::new()
            .write(true)
            .read(true)
            .truncate(true)
            .create(true)
            .open(path);

        match res {
            Ok(file) => {
                return Ok(file);
            }
            Err(err) => {
                return Err(Error::FileOps(err.to_string()));
            }
        };
    }
    fn open_file(path : &str) -> Result<fs::File, Error> {
        let res = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open(path);
        match res {
            Ok(file) => {
                return Ok(file);
            }
            Err(err) => {
                return Err(Error::FileOps(err.to_string()));
            }
        };
    }
    fn update_file(&mut self)  {
        let mut data = self.header.borrow().serialize();
        data.extend_from_slice(
            &self.block_device.borrow().serialize()
        );

        {
            // --- REWRITE: handle error(s)
            let mut file_ref = self.file.borrow_mut(); 
            match file_ref.seek(SeekFrom::Start(0u64)){
                Ok(_) => {
                }
                Err(err) => {
                    println!("Error in updating file: {}", err);
                }
            }
            match file_ref.write_all(&data){
                Ok(_) => {
                }
                Err(err) => {
                    println!("Error in updating file: {}", err);
                }
            }
        }

        if data.len() < fs_base::HEADER_SIZE {
            BlockDevice::append_header_filler(data.len(), self.file.clone());
        }
    }
}


impl Drop for Vfs{
    fn drop(&mut self){
        self.update_file();
    } 
}
