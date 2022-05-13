//! A module for handling fim's editor windows.
//!
//! A window has a single active [`Document`] and can be split vertically or horizontally.
use crate::document::Document;
use crate::terminal::{Position, Size, Terminal};
use std::iter::once;
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
            Ok(())
        }
    }

    fn to_term(&self, x: u16, y: u16) -> Position {
        assert!(x < self.window_size.width && y < self.window_size.height);
        Position{ x: x + self.window_pos.x, y: y + self.window_pos.y }
    }

    fn q_clear(&self, clear_type: ClearType, line: u16, term: &mut Terminal) -> Result<()> { // line is in window coords
        let clear_str: String = once(' ').cycle().take(self.window_size.width as usize).collect();
        match clear_type {
            ClearType::All => {
                term.q(Hide)?.save_cursor();
                for line in 0..self.window_size.height {
                    let Position{ x, y } = self.to_term(0, line);
                    term.cursor_to(x, y).q_move_cursor()?.q(Print(&clear_str))?;
                }
                term.restore_cursor();
                term.q_move_cursor()?.q(Show)?;
            },
            ClearType::Line => {
                term.q(Hide)?.save_cursor();
                let Position{ x, y } = self.to_term(0, line);
                term.cursor_to(x, y).q_move_cursor()?.q(Print(clear_str))?.restore_cursor();
                term.q_move_cursor()?.q(Show)?;
            }
        }
        Ok(())
    }

    fn draw_welcome_screen(&self, term: &mut Terminal) -> Result<()> {
        let height = self.window_size.height;
        let message_len = WELCOME_SIZE as u16;
        let mut message_line: u16 = 0;
        term.q(Hide)?.save_cursor();
        self.q_clear(ClearType::All, 0, term)?;
        for i in 0..height {
            let Position{ x, y } = self.to_term(0, i);
            term.cursor_to(x, y).q_move_cursor()?;
            if message_line < message_len && i == height / 2 - message_len / 2 + message_line {
                // TODO: centered here
                message_line += 1; 
            } else {
                term.q(Print("~".blue()))?;
            }
        }
        term.flush()
    }

    /*
    fn draw_welcome_screen(&self, term: &mut Terminal) -> Result<()> {
        let height = self.terminal.size().height;
        let message_len = WELCOME_SIZE as u16;
        let mut message_line: u16 = 0;
        self.terminal.q(cursor::SavePosition)?.q(cursor::Hide)?.q(Clear(ClearType::All))?;
        for i in 0..(height - 1) {
            if message_line < message_len && i == height / 2 - message_len / 2 + message_line {
                self.terminal.centered_styles("~", &WELCOME_MSG[message_line as usize], "",
                                              Some(ContentStyle::new().blue()), None, None).q()?;
                // self.terminal.q(Print(self.terminal.centered("~", &self.welcome_message[message_line as usize], "") + "\r\n"));
                message_line += 1;
            } else {
                self.terminal.q(Print("~\r\n".blue()))?;
            }
        }
        self.terminal.q(Print("~".blue()))?.q(cursor::RestorePosition)?.q(cursor::Show)?.flush()
    }
    */
}
