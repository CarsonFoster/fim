//! A module for handling the content of files ('documents').

use std::fs::File;
use std::io::Error;
use std::ops::{Bound, Index, RangeBounds};
use std::path::PathBuf;
use std::slice::{Iter, SliceIndex};
use unicode_segmentation::UnicodeSegmentation;

struct Buffer {
    buf: String,
    graphemes: Vec<usize>,
}

impl Buffer {
    pub fn new(buf: String) -> Self {
        let mut graphemes = Vec::new();
        for (idx, g) in buf.as_str().grapheme_indices(true) {
            graphemes.push(idx);
        }
        graphemes.push(buf.len()); // past-the-end grapheme
        Buffer{ buf, graphemes }
    }
}

impl<T> Index<T> for Buffer
where T: RangeBounds<usize> {
    type Output = str;

    fn index(&self, idx: T) -> &Self::Output {
        let begin = match idx.start_bound() {
            Bound::Included(i) => self.graphemes[*i],
            Bound::Excluded(i) => self.graphemes[*i + 1],
            Bound::Unbounded => 0
        };
        let end = match idx.end_bound() {
            Bound::Included(i) => self.graphemes[*i + 1],
            Bound::Excluded(i) => self.graphemes[*i],
            Bound::Unbounded => self.buf.len()
        };
        &self.buf[begin..end]
    }
}

/// Struct that represents a document.
pub struct Document {
    #[doc(hidden)]
    filename: Option<PathBuf>,
    #[doc(hidden)]
    buffers: /* TODO */
}

// TODO: transform incoming tabs according to options??

impl Document {
    /// Create a new Document from a file.
    pub fn new(filename: PathBuf) -> Result<Self, Error> {
        let fin = File::open(filename)?;
    }

}

impl<T> From<T> for Document
where T: AsRef<str>{
    fn from(internal_doc: T) -> Self {
    }
}

impl<'a> IntoIterator for &'a Document {
    type Item = /* TODO &'a item */;
    type IntoIter = Iter<'a, /* TODO item */>;

    fn into_iter(self) -> Self::IntoIter {
        (/* TODO */).iter()
    }
}
