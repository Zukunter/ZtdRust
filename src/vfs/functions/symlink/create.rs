use std::{
    path::Path,
    io::Error,
    os
};
use crate::{
    cc::{self, Cfg}
};

pub fn create_symlink<AsPath1, AsPath2>(from: AsPath1, to: AsPath2, _is_dir: bool) -> Result<(), Error>
where 
    AsPath1: AsRef<Path>,
    AsPath2: AsRef<Path>
{
    let from_ref = from.as_ref();
    let to_ref = to.as_ref();
 
    #[cfg(target_os = "windows")] {
        if _is_dir {
            os::windows::fs::symlink_dir(from_ref, to_ref)?;
        } else {
            os::windows::fs::symlink_file(from_ref, to_ref)?;
        }
    }

    #[cfg(unix)] {
        os::unix::fs::symlink(from_ref, to_ref)?;
    }

    return Ok(());
}

mod macros {
    #[macro_export]
    macro_rules! create_symlink {
        ($link:expr, $to:expr) => {
            $crate::vfs::create_symlink($link, $to, false)
        };
        ($link:expr, $to:expr, $is_dir:expr) => {
            $crate::vfs::create_symlink($link, $to, $is_dir)
        };
    }
    pub use create_symlink;
}

pub use macros::create_symlink;
