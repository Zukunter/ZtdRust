use std::{
    io::Error, 
    path::Path
};
use crate::{
    vfs
};

pub fn copy_quidquid<AsPath1, AsPath2>(from: AsPath1, to: AsPath2) -> Result<(), Error>
where 
    AsPath1: AsRef<Path>,
    AsPath2: AsRef<Path>
{
    let from_ref = from.as_ref();
    let to_ref = to.as_ref();

    if from_ref.is_dir() {
        vfs::copy_dir(from_ref, to_ref)?;
    } else if from_ref.is_symlink() {
        vfs::copy_symlink(from_ref, to_ref)?;
    } else {
      vfs::copy_file(from_ref, to_ref)?;
    }

return Ok(()) ; }
