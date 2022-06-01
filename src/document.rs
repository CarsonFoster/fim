//! A module for handling the content of files ('documents').

use std::io::Error;
use std::path::PathBuf;
use std::slice::{Iter, SliceIndex};
use unicode_segmentation::UnicodeSegmentation;

/// Struct that represents a line of text.
#[derive(Default)]
pub struct Line {
    /// The content of the line.
    pub text: String,
    /// The number of graphemes in the line
    pub length: usize,
}

impl From<&str> for Line {
    fn from(text: &str) -> Self {
        Line{ text: text.to_string(), length: text.graphemes(true).count() }
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

    /// Return the number of lines in the document.
    pub fn num_lines(&self) -> usize {
        self.lines.len()
    }

    /// Get a line at the given (zero-based) index.
    pub fn line(&self, idx: usize) -> Option<&Line> {
        self.lines.get(idx)
    }

    /// Retrieve an iterator into the lines of this document, starting from the given (zero-based)
    /// index, inclusive.
    pub fn iter_from(&self, line_idx: usize) -> Option<Iter<Line>> {
        self.lines.get(line_idx..).map(|s| s.iter())
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
    type IntoIter = Iter<'a, Line>;

    fn into_iter(self) -> Self::IntoIter {
        (&self.lines).iter()
    }
}
