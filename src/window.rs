//! A module for handling fim's editor windows.
//!
//! A window has a single active [`Document`] and can be split vertically or horizontally.
use crate::document::Document;
use crate::terminal::{Position, Size, Terminal};
use crossterm::Result;

/// Struct that represents a position in a document.
///
/// Because it is possible for documents to have more than 2^16 - 1 lines, this needs to have usize
/// fields instead of u16 fields, so we can't reuse [`crate::terminal::Position`].
#[derive(Copy, Clone, Default)]
pub struct DocPosition {
    pub x: usize,
    pub y: usize
}

/// Struct that represents a fim window.
pub struct Window {
    #[doc(hidden)]
    doc: Option<Document>,
    #[doc(hidden)]
    pos_in_doc: DocPosition,
    #[doc(hidden)]
    window_pos: Position,
    #[doc(hidden)]
    window_size: Size,
}

impl Window {
    /// Create a new, full-terminal Window with the default welcome message.
    pub fn default(term: &Terminal) -> Self {
        Window{ doc: None, pos_in_doc: DocPosition::default(), window_pos: Position::default(), window_size: term.size() }
    }

    /// Create a new, full-terminal Window with the contents of the given file.
    pub fn new(filename: &str, term: &Terminal) -> Result<Self> {
        Ok(Window{ doc: Some(Document::new(filename)?), pos_in_doc: DocPosition::default(), window_pos: Position::default(), window_size: term.size() })
    }

    /// Render this window's contents to the terminal screen.
    pub fn render(&self, term: &mut Terminal) -> Result<()> {
        if let Some(doc) = self.doc.as_ref() {
            Ok(())
        } else {
            Ok(())
        }
    }
}
