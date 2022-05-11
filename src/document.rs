//! A module for handling the content of files ('documents').

use std::io::{Error};

/// Struct that represents a document.
pub struct Document {
    #[doc(hidden)]
    filename: Option<String>,
}

impl Document {
    pub fn new(filename: &str) -> Result<Self, Error> {
        Ok(Document{ filename: Some(filename.to_string()) })
    }
}

impl From<&str> for Document {
    fn from(internal_doc: &str) -> Self {
        Document{ filename: None }
    }
}
