//! A module for handling the content of files ('documents').

use std::io::Error;
use std::path::PathBuf;
use std::slice::SliceIndex;

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
    filename: Option<PathBuf>,
    #[doc(hidden)]
    lines: Vec<Line>,
}

impl Document {
    /// Create a new Document from a file.
    pub fn new(filename: PathBuf) -> Result<Self, Error> {
        let text = std::fs::read_to_string(&filename)?;
        Ok(Document{ filename: Some(filename), lines: Self::vec_from_str(&text) })
    }

    /// Get a line at the given (zero-based) index.
    pub fn line(&self, idx: usize) -> Option<&Line> {
        self.lines.get(idx)
    }

    /// Get a unicode character with the given (zero-based) range from the given line.
    pub fn unicode_char<I>(&self, line_idx: usize, char_range: I) -> Option<&<I as SliceIndex<str>>::Output>
    where I: SliceIndex<str>
    {
        self.lines.get(line_idx).map(|l| l.text.get(char_range)).flatten()
    }

    /// Get a ASCII character with the given (zero-based) range from the given line.
    pub fn ascii_char(&self, line_idx: usize, char_idx: usize) -> Option<char> {
        self.lines.get(line_idx).map(|l| l.text.get(char_idx .. char_idx + 1)
            .map(|s| s.chars().next()).flatten()).flatten()
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
