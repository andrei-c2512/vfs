use crate::header::Header;
use std::time::SystemTime;

struct Vfs{
    header : Header,
}

impl Vfs{
    fn open(&self, path : &str) {
         
    }
    fn create_dir(&self, path : &str) {
        let time = SystemTime::now();
        
        //let dir = Directory::from(
         //   INode::from
    }

    fn create(&self, path : &str) {

    }
}
