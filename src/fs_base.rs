use std::fmt;

pub type Permissions = u16;

/*
const READ : u16 = 1 << 0;
const WRITE : u16 = 1 << 1;
*/

pub const BLOCK_DEVICE_PREAMBLE : &str = "BLOCK_DEVICE_PREAMBLE";
pub const HEADER_TAIL : &str = "HEADER_END";
pub const BLOCK_CAPACITY : usize = 8 * 4;
pub const MAX_PROCESS_CAPACITY : usize= BLOCK_CAPACITY * 2;

/* 8kb for header */
pub const HEADER_SIZE : usize = 1024 * 1;

pub enum Error{
    InvalidPreamble(String),
    InvalidStringBuffer(String),
    EmptyPath(String),
    InvalidPath(String),
    InvalidDirective(String),
    BadDeser(String),
    Unreachable(String),
    FileOps(String),
}


impl fmt::Display for Error{
    // -- REWRITE: Give each error type a proper message header (or whatever the preceeding thing
    // is called. Also review error messages in general
    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
        match self{
            Error::InvalidPreamble(e) => write!(f, "Invalid preamble: {}", e),
            Error::InvalidStringBuffer(e) => write!(f, "Invalid string buffer: {}", e),
            Error::EmptyPath(e) => write!(f, "Empty path: {}", e),
            Error::InvalidPath(e) => write!(f, "Invalid path: {}", e),
            Error::InvalidDirective(e) => write!(f, "Invalid directive: {}", e),
            Error::BadDeser(e) => write!(f, "Deserialization error: {}", e),
            Error::Unreachable(e) => write!(f, "Encountered unreachable code: {}", e),
            Error::FileOps(e) => write!(f, "File operation error: {}", e),
        }
    }
}
