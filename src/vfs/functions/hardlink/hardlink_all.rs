use std::{
    fs,
    path::Path,
    io::Error
};
use crate::vfs;

pub fn hard_link_all<AsPath1, AsPath2>(link: AsPath1, to: AsPath2) -> Result<(), Error>
where 
    AsPath1: AsRef<Path>,
    AsPath2: AsRef<Path> 
{
    let link_ref = link.as_ref();
    let to_ref = to.as_ref();

    vfs::create_parent_all(link_ref)?;

    fs::hard_link(to_ref, link_ref)?;

return Ok(()) ; }
