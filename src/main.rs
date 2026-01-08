pub mod block_device;
pub mod directory;
pub mod file;
pub mod fs_base;
pub mod header;
pub mod inode;
pub mod ops;
pub mod printer;
pub mod serde;
pub mod string_buffer;
pub mod test;
pub mod traits;
pub mod util;
pub mod vfs;

use crate::fs_base::Error;

fn main() -> Result<(), Error> {
    test::run_all()?;
    Ok(())
}
