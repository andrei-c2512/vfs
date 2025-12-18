pub enum Error{
    InvalidPreamble(String),
    InvalidStringBuffer(String),
    EmptyPath(String),
    InvalidPath(String),
    InvalidDirective(String),
}

