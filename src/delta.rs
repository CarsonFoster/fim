//! Module to deal with fim's 'differential' files (analogous to vim's swap files).
//!
//! (This module is currently under construction)
//!
//! Differentials are records of the (unsaved) changes to a file. The differential files on disk
//! normally have an extension of `.fdiff`.
use bincode::{serialize, deserialize};
use serde::{Serialize, Deserialize};
use std::ffi::OsString;
use std::fs::{read, write};
use std::io::{Error, ErrorKind, Result};
use std::path::{Path, PathBuf};

/// Struct representing a change to a file open in fim.
#[derive(Serialize, Deserialize)]
pub struct Delta {
    
}

/// Struct representing all of the changes to a file open in fim.
#[derive(Serialize, Deserialize)]
pub struct Differential {
    #[doc(hidden)]
    deltas: Vec<Delta>,
    #[doc(hidden)]
    file: PathBuf,
}

impl Differential {
    /// Create a new Differential with no changes from a file path.
    pub fn new<P: AsRef<Path>>(file: P) -> Result<Self> {
        Ok(Differential{ deltas: Vec::new(), file: file.as_ref().canonicalize()? })
    }

    /// Read a Differential into memory from the differential file.
    ///
    /// Differential files normally have an extension of `.fdiff`.
    pub fn from_backup<P: AsRef<Path>>(backup_file: P) -> Result<Self> {
        let bytes = read(backup_file)?;
        deserialize(&bytes[..]).map_err(|e| Error::new(ErrorKind::Other, e))
    }
 
    /*
    // TODO:
    // Will we save files from a differential?? Or directly from the document?
    pub fn save(self) -> Result<()> {
        Ok(())
    }
    */

    /// Write this differential to disk.
    ///
    /// The resulting differential file has the same path as the main file, but has an extension of
    /// `.fdiff`.
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
