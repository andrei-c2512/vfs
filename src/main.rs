pub mod util;
pub mod string_buffer;
pub mod serde;
pub mod directory;
pub mod fs_base;
pub mod file;
pub mod traits;
pub mod header;

use std::collections::HashMap;

use crate::util::date_time::DateTime;
use crate::util::string_helper;
use crate::string_buffer::StringBuffer;
use crate::directory::Directory;
use crate::file::File;
use crate::traits::Directive;
use crate::header::Header;



fn main() {
    println!("Hello, world!");
}
