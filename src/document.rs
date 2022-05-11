//! A module for handling the content of files ('documents').

use std::io::{Error};

/// Struct that represents a line of text.
#[derive(Default)]
pub struct Line {
    text: String,
}

impl From<&str> for Line {
    fn from(text: &str) -> Self {
        Line{ text: text.to_string() }
    }
}

/// Struct that represents a document.
pub struct Document {
    #[doc(hidden)]
    filename: Option<String>,
    #[doc(hidden)]
    lines: Vec<Line>,
}

impl Document {
    /// Create a new Document from a file.
    pub fn new(filename: &str) -> Result<Self, Error> {
        let text = std::fs::read_to_string(filename)?;
        Ok(Document{ filename: Some(filename.to_string()), lines: Self::vec_from_str(&text) })
    }

    /// Get a line at the given (zero-based) index.
    pub fn line(&self, idx: usize) -> Option<&Line> {
        self.lines.get(idx)
    }

    fn vec_from_str(text: &str) -> Vec<Line> {
        text.lines().map(|s| Line::from(s)).collect()
    }
}

impl From<&str> for Document {
    fn from(internal_doc: &str) -> Self {
        Document{ filename: None, lines: Self::vec_from_str(internal_doc) }
    }
}

impl<'a> IntoIterator for &'a Document {
    type Item = &'a Line;
    type IntoIter = std::slice::Iter<'a, Line>;

    fn into_iter(self) -> Self::IntoIter {
        (&self.lines).iter()
    }
}
