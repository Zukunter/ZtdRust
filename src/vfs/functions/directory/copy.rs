use std::{
    io::Error, 
    path::Path,
    fs
};
use crate::{
    vfs
};

pub fn copy_dir<AsPath1, AsPath2>(from: AsPath1, to: AsPath2) -> Result<(), Error>
where 
    AsPath1: AsRef<Path>,
    AsPath2: AsRef<Path>
{
    let from_ref = from.as_ref();
    let mut to_route = to.as_ref().to_owned();
    to_route.push(from_ref);

    fs::create_dir(&to_route)?;

    let entries = fs::read_dir(from_ref)?;

    for entry in entries {
        let entry = entry?;
        vfs::copy_quidquid(entry.path(), &to_route)?;
    }

return Ok(()) ; }
