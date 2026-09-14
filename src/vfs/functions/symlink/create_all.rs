use crate::vfs;
use std::{
    path::Path,
    io::Error
};

pub fn create_symlink_all<AsPath1, AsPath2>(link: AsPath1, to: AsPath2, is_dir: bool) -> Result<(), Error>
where
    AsPath1: AsRef<Path>,
    AsPath2: AsRef<Path>,
{
    let link_ref = link.as_ref();
    let to_ref = to.as_ref();
    vfs::create_parent_all(link_ref)?;
    vfs::create_symlink(link_ref, to_ref, is_dir)?;
    Ok(())
}

mod macros {
    #[macro_export]
    macro_rules! create_symlink_all {
        ($link:expr, $to:expr) => {
            $crate::vfs::create_symlink_all($link, $to, false)
        };
        ($link:expr, $to:expr, $is_dir:expr) => {
            $crate::vfs::create_symlink_all($link, $to, $is_dir)
        };
    }
    pub use create_symlink_all;
}

pub use macros::create_symlink_all;
