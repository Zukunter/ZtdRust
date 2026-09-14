use std::{
    path::Path,
    io::Error
};
#[cfg(unix)]
    use std::os::unix::fs::symlink;
#[cfg(windows)]
    use std::os::windows::fs::symlink_file;
#[cfg(windows)]
    use std::os::windows::fs::symlink_dir;

pub fn create_symlink<AsPath1, AsPath2>(link: AsPath1, to: AsPath2, _is_dir: bool) -> Result<(), Error>
where 
    AsPath1: AsRef<Path>,
    AsPath2: AsRef<Path>
{
    let link_ref = link.as_ref();
    let to_ref = to.as_ref();
    #[cfg(windows)]
        if _is_dir {
            symlink_dir(to_ref, link_ref)?;
        } else {
            symlink_file(to_ref, link_ref)?;
        }
    #[cfg(unix)]
        symlink(to_ref, link_ref)?;
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
