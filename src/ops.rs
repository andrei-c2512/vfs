use crate::block_device::BlockDevice;
use crate::fs_base::{BUFFERED_IO_LIMIT, Error, READ_TO_STRING_LIMIT};
use crate::header::{Header, Node};
use crate::util::date_time::DateTime;
//use crate::file::FileData;

use std::cell::RefCell;
use std::fs;
use std::io;
use std::io::{Read, Write};
use std::rc::Rc;

pub struct File {
    file_id: u32,
    header: Rc<RefCell<Header>>,
    block_device: Rc<RefCell<BlockDevice>>,
    vfs_file: Rc<RefCell<fs::File>>,

    buffer: Vec<u8>,
    last_block_index: u32,
}

impl File {
    pub fn from(
        file_id: u32,
        header: Rc<RefCell<Header>>,
        block_device: Rc<RefCell<BlockDevice>>,
        vfs_file: Rc<RefCell<fs::File>>,
    ) -> Self {
        Self {
            file_id,
            header,
            block_device,
            vfs_file,
            buffer: Vec::new(),
            last_block_index: 0,
        }
    }
    pub fn write_all(&mut self, buffer: &[u8]) -> Option<Error> {
        let node = &mut self.header.borrow_mut().node_buffer[self.file_id as usize];

        if let Node::File(file_data) = node {
            self.block_device.borrow_mut().write(
                buffer,
                &mut file_data.block_indices,
                self.vfs_file.clone(),
            );
            file_data.inode.last_modified = DateTime::now();
            file_data.inode.size = buffer.len();
        }
        None
    }
    // imma do this later to make this showcase worthy
    /*
    pub fn write_chunk(&mut self, buffer : &[u8]) -> Result<(), Error> {
        let node = &mut self.header.borrow_mut().node_buffer[self.file_id as usize];
        let file_data = match node {
            Node::File(file_data) => file_data,
            Node::Directory(_) => {
                return Err(Error::BadCall(
                    "Attempted to use a directory index in file operations".to_string(),
                ));
            }
        };

        self.block_device.borrow_mut().append(
            buffer,
            &mut file_data.block_indices,
            self.vfs_file.clone(),
            file_data.inode.size,
        );

        file_data.inode.size += buffer.len();
        Ok(())
    }
    */
    pub fn write_from_file(&mut self, path: &str) -> Result<(), Error> {
        let file = match fs::File::open(path) {
            Ok(file) => file,
            Err(err) => {
                return Err(Error::FileOps(format!(
                    "Unable to write from file '{}': {}",
                    path, err
                )));
            }
        };
        let mut reader = io::BufReader::new(file);
        let node = &mut self.header.borrow_mut().node_buffer[self.file_id as usize];

        let file_data = match node {
            Node::File(file_data) => file_data,
            Node::Directory(_) => {
                return Err(Error::BadCall(
                    "Attempted to use a directory index in file operations".to_string(),
                ));
            }
        };
        // we overwrite the existing buffers!
        file_data.inode.size = 0;
        self.buffer.resize(BUFFERED_IO_LIMIT, b' ');
        let mut total_bytes_read = 0;

        loop {
            let bytes_read = match reader.read(&mut self.buffer) {
                Ok(bytes_read) => bytes_read,
                Err(err) => {
                    return Err(Error::FileOps(format!(
                        "Error in using buffered reader: {}",
                        err
                    )));
                }
            };
            if bytes_read == 0 {
                break;
            }
            self.block_device.borrow_mut().append(
                &self.buffer[..bytes_read],
                &mut file_data.block_indices,
                self.vfs_file.clone(),
                total_bytes_read,
            );
            total_bytes_read += bytes_read;
        }

        file_data.inode.size = total_bytes_read;

        Ok(())
    }
    pub fn append(&self, _bytes: &[u8]) -> Option<Error> {
        panic!("Called unimplemented function 'append'");
    }
    /* This endless match-pattern stuff I see is only and only because I don't store directories
     * and files in different lists. Bruh */
    pub fn read_to_string(&mut self, data: &mut String) -> Result<(), Error> {
        let node = &mut self.header.borrow_mut().node_buffer[self.file_id as usize];
        if let Node::File(file_data) = node {
            if file_data.inode.size >= READ_TO_STRING_LIMIT {
                return Err(Error::FileOps(
                    "File too big to read into a string. Try buffered reading".to_string(),
                ));
            }

            self.block_device.borrow_mut().read_to_string(
                data,
                &file_data.block_indices,
                file_data.inode.size,
                self.vfs_file.clone(),
            );
        }
        Ok(())
    }
    pub fn read(&mut self, data: &mut Vec<u8>) {
        let node = &mut self.header.borrow_mut().node_buffer[self.file_id as usize];
        if let Node::File(file_data) = node {
            self.block_device.borrow_mut().read(
                data,
                &file_data.block_indices,
                file_data.inode.size,
                self.vfs_file.clone(),
            );
        }
    }
    pub fn read_chunk(&mut self, buffer: &mut [u8; BUFFERED_IO_LIMIT]) -> Result<usize, Error> {
        let mut bd_ref = self.block_device.borrow_mut();
        let node = &mut self.header.borrow_mut().node_buffer[self.file_id as usize];
        if let Node::File(file_data) = node {
            let res = bd_ref.read_from(
                buffer,
                &file_data.block_indices,
                self.vfs_file.clone(),
                file_data.inode.size,
                &mut self.last_block_index,
            );
            return res;
        }
        Err(Error::BadCall(
            "Index invalidated/corrupted. It now points to a directory".to_string(),
        ))
    }
    pub fn write_to_os(&mut self, path: &str) -> Result<(), Error> {
        let mut buffer = [b' '; BUFFERED_IO_LIMIT];

        let mut file = match fs::OpenOptions::new().write(true).truncate(true).open(path) {
            Ok(file) => file,
            Err(err) => {
                return Err(Error::FileOps(format!(
                    "Error in opening file '{}': {}",
                    path, err
                )));
            }
        };

        loop {
            let bytes_read = self.read_chunk(&mut buffer)?;
            if bytes_read == 0 {
                break;
            }
            _ = file.write_all(&buffer);
        }
        Ok(())
    }
    pub fn write_gibberish(&mut self) -> Result<(), Error> {
        let chunk = vec![b'A'; BUFFERED_IO_LIMIT]; // 1MB chunks
        let n_chunks = 61035; // achieves ~32 GB
        //
        let node = &mut self.header.borrow_mut().node_buffer[self.file_id as usize];
        let mut total_bytes_read = 0;

        let file_data = match node {
            Node::File(file_data) => file_data,
            Node::Directory(_) => {
                return Err(Error::BadCall(
                    "Attempted to use a directory index in file operations".to_string(),
                ));
            }
        };

        file_data.inode.size = 0;
        for _ in 0..n_chunks {
            self.block_device.borrow_mut().append(
                &chunk,
                &mut file_data.block_indices,
                self.vfs_file.clone(),
                total_bytes_read,
            );
            total_bytes_read += chunk.len();
        }

        file_data.inode.size = total_bytes_read;
        Ok(())
    }
}

pub struct Directory {
    /*
    dir_id : u32,
    header : Rc<RefCell<Header>>,
    block_device : Rc<RefCell<BlockDevice>>,
    vfs_file : Rc<RefCell<fs::File>>,
    */
}
impl Directory {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for Directory {
    fn default() -> Self {
        Self::new()
    }
}
