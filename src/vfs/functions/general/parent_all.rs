use std::{
    path::Path,
    fs::create_dir_all,
    io::Error
};

pub fn create_parent_all<AsPath>(path: AsPath) -> Result<(), Error> 
where 
    AsPath: AsRef<Path>
{
    let path_ref = path.as_ref();

    if let Some(parent_path) = path_ref.parent() {

        if !parent_path.exists() || !parent_path.is_dir() {
            create_dir_all(parent_path)?;
        }
    }

return Ok(()) ; }
