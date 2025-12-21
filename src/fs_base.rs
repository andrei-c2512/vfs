use std::fmt;

pub type Permissions = u16;

const READ : u16 = 1 << 0;
const WRITE : u16 = 1 << 1;

pub enum Error{
    InvalidPreamble(String),
    InvalidStringBuffer(String),
    EmptyPath(String),
    InvalidPath(String),
    InvalidDirective(String),
    BadDeser(String),
    Unreachable(String),
}


impl fmt::Display for Error{
    // -- REWRITE: Give each error type a proper message header (or whatever the preceeding thing
    // is called. Also review error messages in general
    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
        match self{
            Error::InvalidPreamble(e) => write!(f, "{}", e),
            Error::InvalidStringBuffer(e) => write!(f, "{}", e),
            Error::EmptyPath(e) => write!(f, "{}", e),
            Error::InvalidPath(e) => write!(f, "{}", e),
            Error::InvalidDirective(e) => write!(f, "{}", e),
            Error::BadDeser(e) => write!(f, "{}", e),
            Error::Unreachable(e) => write!(f, "{}", e),
        }
    }
}
