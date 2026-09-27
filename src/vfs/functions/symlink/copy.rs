use std::{
    io::Error, 
    path::Path,
    os
};
use crate::{
    vfs,
    cc
};

pub fn copy_symlink<AsPath1, AsPath2>(from: AsPath1, to: AsPath2) -> Result<(), Error>
where 
    AsPath1: AsRef<Path>,
    AsPath2: AsRef<Path>
{
    let from_ref = from.as_ref();
    let to_ref = to.as_ref();

return Ok(()) ; }
