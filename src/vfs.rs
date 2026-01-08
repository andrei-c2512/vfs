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
use crate::util::string_helper;

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

        let file = Self::open_os_file(path)?;
        let shared_file = Rc::new(RefCell::new(file));


        Ok(Self{header : shared_header, block_device : shared_bd, file : shared_file})
    }
    pub fn read_dir(&self, path : &str) -> Result<Vec<u32>, Error>{ 
        let header_ref = self.header.borrow_mut();

        let engine_path = Self::to_engine_path(path);
        let node_id = header_ref.navigate(&engine_path)?;
        let node = &header_ref.node_buffer[node_id as usize];        
            
        match node{
            Node::Directory(dir_data) => {
                Ok(dir_data.children.clone())
            }
            Node::File(_) => {
                Err(Error::BadCall("Attempted to call read_dir on a file".to_string()))
            }
        }
    }
    pub fn create_dir(&mut self, path : &str) -> Result<ops::Directory, Error>{
        let engine_path = Self::to_engine_path(path);
        let (last_directive, path_to_directive) = self.split_from_cwd(&engine_path);
        
        let _dir_id = {
            let mut header_ref = self.header.borrow_mut();
            let name_id = header_ref.str_buffer.add(last_directive);
            let dir = DirectoryData::from(
                INode::from(name_id, 0, DateTime::now(), DateTime::now(), 0), Vec::new()
            ); 

            match header_ref.add_node(path_to_directive, name_id, Node::Directory(dir)) {
                Err(err) => { return Err(err); }
                Ok(dir_id) => { dir_id }
            }
        };
        self.update_os_file();

        Ok(ops::Directory::new())
    }
    pub fn create(&mut self, path : &str) -> Result<ops::File, Error> { 
        match self.open_file(path){
            Ok(file) => {
                return Ok(file);
            }
            _ => {}
        };
        let engine_path = Self::to_engine_path(path);
        let (last_directive, path_to_directive) = self.split_from_cwd(&engine_path);


        let file_id = {
            let mut header_ref = self.header.borrow_mut();
            let name_id = header_ref.str_buffer.add(last_directive);
            let file : file::FileData = file::FileData::from(
                INode::from(name_id, 0, DateTime::now(), DateTime::now(), 0), Vec::new()
            ); 

            match header_ref.add_node(path_to_directive, name_id, Node::File(file)) {
                Err(err) => { return Err(err); }
                Ok(file_id) => { file_id }
            }
        }; 
        self.update_os_file();

        Ok(
            ops::File::from(file_id, self.header.clone(), self.block_device.clone(), self.file.clone())
        )
    }
    pub fn open_file(&self, path : &str) -> Result<ops::File, Error> {
        let header_ref = self.header.borrow_mut();
        let node_id = header_ref.navigate(&Self::to_engine_path(path))?;
        let node = &header_ref.node_buffer[node_id as usize];        
            
        match node{
            Node::Directory(_) => {
                Err(Error::BadCall("Attempted to call open_file on a directory".to_string()))
            }
            Node::File(_) => {
                Ok(
                    ops::File::from(node_id, self.header.clone(), self.block_device.clone(), self.file.clone())
                    )
            }
        }
    }
    pub fn open_entry(&self, entry : u32) -> Result<ops::File, Error> {
        let header_ref = self.header.borrow_mut();
        let node_id = entry;
        let node = &header_ref.node_buffer[node_id as usize];        
            
        match node{
            Node::Directory(_) => {
                Err(Error::BadCall("Attempted to call open_file on a directory".to_string()))
            }
            Node::File(_) => {
                Ok(
                    ops::File::from(node_id, self.header.clone(), self.block_device.clone(), self.file.clone())
                )
            }
        }
    }
    // testing for < 10MB files
    pub fn copy_into_vfs(&mut self, path : &str, vfs_path : &str) -> Result<(), Error> {
        let bytes = match fs::read(path){
            Ok(bytes) => { bytes}
            // --- RETHINK: 
            // hope it is not confusing for me later on as to if it's vfs path or my system path
            // may need to create a seperate error
            Err(err) => { return Err(Error::FileOps(string_helper::fmt_file_error(&err.to_string(), path))); } 
        };
        
        let mut vfs_file = self.create(vfs_path)?;
        vfs_file.write_all(&bytes);
        Ok(())
    }
    // I want to let the user dictate the flags of the copying
    pub fn copy_from_vfs(&mut self, vfs_path : &str, os_file : &mut fs::File) -> Result<(), Error> {
        let mut file = self.open_file(vfs_path)?;
        let mut data = Vec::new();
        file.read(&mut data);

        match os_file.write(&data) {
            Err(err) => { Err(Error::FileOps(err.to_string())) }
            _ => { Ok(()) } 
        }
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
            0 => { (path, path) }
            _ => { (&path[last_slash_ind + 1..], &path[..last_slash_ind])}
        }
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
                Ok(file)
            }
            Err(err) => {
                Err(Error::FileOps(err.to_string()))
            }
        }
    }
    fn open_os_file(path : &str) -> Result<fs::File, Error> {
        let res = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open(path);
        match res {
            Ok(file) => {
                Ok(file)
            }
            Err(err) => {
                Err(Error::FileOps(err.to_string()))
            }
        }
    }
    fn update_os_file(&mut self)  {
        let mut data = self.header.borrow().serialize();
        data.extend_from_slice(
            &self.block_device.borrow().serialize()
        );
        {
            // --- REWRITE: handle error(s)
            let mut file_ref = self.file.borrow_mut(); 
            if let Err(err) = file_ref.seek(SeekFrom::Start(0u64)){
                println!("Error in updating file: {}", err);
            } 
            if let Err(err) = file_ref.write_all(&data){
                    println!("Error in updating file: {}", err);
            }
        }

        if data.len() < fs_base::HEADER_SIZE {
            BlockDevice::append_header_filler(data.len(), self.file.clone());
        }
    }
    fn to_engine_path(path : &str) -> String{
        "@/".to_string() + path
    }
}


impl Drop for Vfs{
    fn drop(&mut self){
        self.update_os_file();
    } 
}
