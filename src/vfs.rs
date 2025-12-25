use crate::header::{Header,Node};
use crate::directory::DirectoryData;
use crate::file;
use crate::inode::INode;
use crate::util::date_time::DateTime;
use crate::fs_base::Error;
//use crate::string_buffer::StringBuffer;
use crate::printer;
use crate::traits::Serde;
use crate::block_device::BlockDevice;
use crate::ops;

use std::fs;
use std::io::Write;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Vfs{
    path : String,
    header : Rc<RefCell<Header>>, 
    block_device : Rc<RefCell<BlockDevice>>,
    file : Rc<RefCell<fs::File>>,
}

impl Vfs{
    pub fn new(path : &str) -> Result<Self,Error>{        
        let file = Self::open_file(path)?;
        let shared_file = Rc::new(RefCell::new(file));

        Ok(
            Self{ path : path.to_string(), 
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


        Ok(Self{ path : path.to_string(), header : shared_header, block_device : shared_bd, file : shared_file})
    }
    pub fn create_dir(&mut self, path : &str) -> Option<Error>{
        let (last_directive, path_to_directive) = self.split_from_cwd(path);

        let mut header_ref = self.header.borrow_mut();
        let name_id = header_ref.str_buffer.add(last_directive);
        let dir = DirectoryData::from(
            INode::from(name_id, 0, DateTime::now(), DateTime::now(), 0), Vec::new()
        ); 

        return header_ref.add_node(path_to_directive, name_id, Node::Directory(dir));
    }
    pub fn create(&mut self, path : &str) -> Result<ops::File, Error> { 
        let (last_directive, path_to_directive) = self.split_from_cwd(path);

        let mut header_ref = self.header.borrow_mut();
        let name_id = header_ref.str_buffer.add(last_directive);
        let file : file::FileData = file::FileData::from(
            INode::from(name_id, 0, DateTime::now(), DateTime::now(), 0), Vec::new()
        ); 

        let err = header_ref.add_node(path_to_directive, name_id, Node::File(file));
        match err {
            Some(err) => { return Err(err) }
            _ => {}
        };

        Ok(
            ops::File::from(0, self.header.clone(), self.block_device.clone(), self.file.clone())
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
    fn open_file(path : &str) -> Result<fs::File, Error> {
        match fs::File::open(path){
            Ok(file) => {
                return Ok(file);
            }
            Err(err) => {
                return Err(Error::FileOps(err.to_string()));
            }
        };
    }
}

impl Drop for Vfs{
    fn drop(&mut self){
        let data = self.header.borrow().serialize();
        self.file.borrow_mut().write_all(&data);
    } 
}
