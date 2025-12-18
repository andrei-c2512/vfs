pub type Permissions = u16;

const READ : u16 = 1 << 0;
const WRITE : u16 = 1 << 1;

pub enum Error{
    InvalidPreamble(String),
    InvalidStringBuffer(String),
    EmptyPath(String),
    InvalidPath(String),
    InvalidDirective(String),
}



