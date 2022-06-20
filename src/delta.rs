use bincode::{serialize, deserialize};
use serde::{Serialize, Deserialize};
use std::ffi::OsString;
use std::fs::{read, write};
use std::io::{Error, ErrorKind, Result};
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize)]
pub struct Delta {
    
}

#[derive(Serialize, Deserialize)]
pub struct Differential {
    #[doc(hidden)]
    deltas: Vec<Delta>,
    #[doc(hidden)]
    file: PathBuf,
}

impl Differential {
    pub fn new<P: AsRef<Path>>(file: P) -> Result<Self> {
        Ok(Differential{ deltas: Vec::new(), file: file.as_ref().canonicalize()? })
    }

    pub fn from_backup<P: AsRef<Path>>(backup_file: P) -> Result<Self> {
        let bytes = read(backup_file)?;
        deserialize(&bytes[..]).map_err(|e| Error::new(ErrorKind::Other, e))
    }
 
    pub fn save(self) -> Result<()> {
        Ok(())
    }

    pub fn backup(&self) -> Result<()> {
        let bytes = serialize(self).map_err(|e| Error::new(ErrorKind::Other, e))?;
        let mut backup_name = OsString::from(".");
        let filename = self.file.file_name().map_or_else(|| Err(Error::new(ErrorKind::Other, format!("could not serialize, {} has no file name", self.file.display()))), |f| Ok(f))?;
        backup_name.push(filename);
        backup_name.push(".fdiff");
        write(self.file.with_file_name(backup_name), &bytes[..])
    }

    // TODO:
    // pub fn recover(&self) -> Result<Document> {}
}
