//! A module for handling fim's editor windows.
//!
//! A window has a single active [`Document`] and can be split vertically or horizontally.
use crate::document::Document;
use crate::terminal::{Position, Size, Terminal};
use crossterm::{
    Result,
    cursor::{Hide, Show},
    style::{Print, Stylize},
};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const WELCOME_SIZE: usize = 4;
lazy_static! {
    static ref WELCOME_MSG: [String; WELCOME_SIZE] = ["FIM - Foster's vIM-like editor".into(), String::new(), format!("Version {}", VERSION), "by Carson Foster".into()];
}

/// Struct that represents a position in a document.
///
/// Because it is possible for documents to have more than 2^16 - 1 lines, this needs to have usize
/// fields instead of u16 fields, so we can't reuse [`crate::terminal::Position`].
#[derive(Copy, Clone, Default)]
pub struct DocPosition {
    pub x: usize,
    pub y: usize
}

#[non_exhaustive]
#[derive(Copy, Clone)]
enum ClearType {
   All,
   Line
}

/// Struct that represents a fim window.
pub struct Window {
    #[doc(hidden)]
    doc: Option<Document>,
    #[doc(hidden)]
    pos_in_doc: DocPosition,
    #[doc(hidden)]
    window_pos: Position, // location of (0, 0) in window
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
            self.draw_welcome_screen(term)
        }
    }
    
    // NOTE: when you implement splitting, make sure that all split windows have
    // documents, and that you change the existing window to have a new blank document
    // if it doesn't have a document, so that the invariants for draw_welcome_screen are
    // maintained.

    fn to_term(&self, x: u16, y: u16) -> Position {
        assert!(x < self.window_size.width && y < self.window_size.height);
        Position{ x: x + self.window_pos.x, y: y + self.window_pos.y }
    }

    fn q_clear(&self, clear_type: ClearType, line: u16, term: &mut Terminal) -> Result<()> { // line is in window coords
        // does not change cursor visibility
        let clear_str = " ".repeat(self.window_size.width as usize);
        match clear_type {
            ClearType::All => {
                term.save_cursor();
                for line in 0..self.window_size.height {
                    let Position{ x, y } = self.to_term(0, line);
                    term.cursor_to(x, y).q_move_cursor()?.q(Print(&clear_str))?;
                }
                term.restore_cursor();
                term.q_move_cursor()?;
            },
            ClearType::Line => {
                term.save_cursor();
                let Position{ x, y } = self.to_term(0, line);
                term.cursor_to(x, y).q_move_cursor()?.q(Print(clear_str))?.restore_cursor();
                term.q_move_cursor()?;
            }
        }
        Ok(())
    }

    fn center_welcome(&self, idx: usize, term: &mut Terminal) -> Result<()> {
        let line = &WELCOME_MSG[idx];
        let width = self.window_size.width as usize;
        if width <= line.len() { // can fit less than or equal to main text
            term.q(Print(&line[..width]))?;
        } else if width == line.len() + 1 { // can fit exactly line and tilde
            term.q(Print("~".blue()))?.q(Print(line))?;
        } else { // can fit line, tilde, and padding
            // extra padding on right
            let left = (width - 1 - line.len()) / 2;
            let right = width - 1 - line.len() - left;
            let left = " ".repeat(left);
            let right = " ".repeat(right);
            term.q(Print("~".blue()))?.q(Print(left))?.q(Print(line))?.q(Print(right))?;
        }
        Ok(())
    }

    fn draw_welcome_screen(&self, term: &mut Terminal) -> Result<()> {
        // cannot be called when there is a Document
        // this means that it also can only be called on a full-terminal window
        if self.doc.is_some() {
            return Ok(())
        }
        let message_len = WELCOME_SIZE as u16;
        let message_begin_line = if self.window_size.height / 2 < message_len / 2 { 0 } else {
            self.window_size.height / 2 - message_len / 2
        };
        let mut message_line: u16 = 0;
        term.q(Hide)?.save_cursor();
        self.q_clear(ClearType::All, 0, term)?;
        for i in 0..(self.window_size.height - 1) {
            let Position{ x, y } = self.to_term(0, i);
            term.cursor_to(x, y).q_move_cursor()?;
            if message_line < message_len && i == message_begin_line + message_line {
                self.center_welcome(message_line as usize, term)?;
                message_line += 1; 
            } else {
                term.q(Print("~".blue()))?;
            }
        }
        term.restore_cursor();
        term.q_move_cursor()?.q(Show)?.flush()
    }
}
