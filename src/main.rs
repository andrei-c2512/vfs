pub mod util;
pub mod string_buffer;
pub mod directory;
pub mod fs_base;
pub mod file;
pub mod traits;
pub mod header;
pub mod inode;
pub mod serde;
pub mod vfs;
pub mod printer;
pub mod test;
pub mod ops;
pub mod block_device;

use crate::fs_base::Error;

fn main() -> Result<(), Error>{
    test::run_all()?;
    Ok(())
}
