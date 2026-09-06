use std::{
    ffi::{
        OsString
    }, 
    path::{
        Path, PathBuf
    }
};

pub trait PathToString {
    fn try_to_string(&self) -> Result<String, OsString>;
}

impl PathToString for PathBuf {
    fn try_to_string(&self) -> Result<String, OsString> {
        self.as_path().try_to_string()
    }
}

impl PathToString for Path {
    fn try_to_string(&self) -> Result<String, OsString> {
        let str = self.to_str()
            .map(String::from);
        match str {
            Some(val) => return Ok(val),
            None => return Err(self.as_os_str().to_owned())
        }
    }
}
