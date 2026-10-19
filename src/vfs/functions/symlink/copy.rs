use std::{
    io::{
        Error, 
        ErrorKind
    },
    path::Path,
    fs
};
use crate::vfs;

pub fn copy_symlink<AsPath1, AsPath2>(from: AsPath1, to: AsPath2) -> Result<(), Error>
where 
    AsPath1: AsRef<Path>,
    AsPath2: AsRef<Path>,
{
    let from_ref = from.as_ref();
    let to_ref = to.as_ref();

    let mut _is_dir = false;

    #[cfg(windows)] {
        use std::os::windows::fs::MetadataExt;
        
        let metadata = fs::symlink_metadata(from_ref)?;
        let attributes = metadata.file_attributes();
        const FILE_ATTRIBUTE_DIRECTORY: u32 = 0x00000010;

        is_dir = (attributes & FILE_ATTRIBUTE_DIRECTORY) != 0;
    }

    let target = fs::read_link(from_ref)?;

    let to_name = from_ref.file_name().ok_or_else(|| {
        Error::new(
            ErrorKind::InvalidData,
            format!(
                "OsString {:?} has no valid path equivalence on this platform",
                from_ref
            ),
        )
    })?;

    let destination = to_ref.join(to_name);

    vfs::create_symlink!(destination, target, _is_dir)?;

    Ok(())
}
