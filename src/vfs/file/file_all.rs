use std::{
    path::{
        Path
    },
    fs::{
        File
    },
    io::{
        Error
    }
};

use crate::vfs::create_parent_all;

pub fn create_file_all<AsPath>(path: AsPath, force: bool) -> Result<File, Error>
where
    AsPath: AsRef<Path>
{
    let path_ref = path.as_ref();

    let _ = create_parent_all(path_ref)?;

    let file = if force { 
        File::create_new(path_ref)?
    } else {
        File::create(path_ref)?
    };

return Ok(file) ; }
