use std::{
    path::Path,
    io::Error
};
use crate::{
    vfs
};

pub fn copy_symlink_all<AsPath1, AsPath2>(from: AsPath1, to: AsPath2) -> Result<(), Error>
where 
    AsPath1: AsRef<Path>,
    AsPath2: AsRef<Path>
{
    let from_ref = from.as_ref();
    let to_ref = to.as_ref();

    vfs::create_parent_all(to_ref)?;
    vfs::copy_symlink(from_ref, to_ref)?;
    
return Ok(()) ; }
