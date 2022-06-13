//! A module for handling fim's editor windows.
//!
//! A window has a single active [`Document`] and can be split vertically or horizontally.
use crate::config::options::{LineNumbers, Options};
use crate::document::Document;
use crate::terminal::{Position, Size, Terminal};
use crossterm::{
    Result,
    cursor::{Hide, Show},
    style::{Print, StyledContent, Stylize},
};
use std::cmp::{max, min};
use std::iter::{once, repeat};
use std::path::PathBuf;
use unicode_segmentation::UnicodeSegmentation;

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

#[derive(Copy, Clone)]
enum WindowLineType {
    DocLine(usize),
    WrappedLine,
    Tilde
}

#[non_exhaustive]
#[derive(Copy, Clone)]
enum ClearType {
   All,
   LineNumbers,
}

#[derive(Copy, Clone)]
struct WindowLineProperties {
    pub lines: usize,    // number of window lines the Line takes up
    pub graphemes: u16 // number of graphemes on the last window line (all others must have raw_window_size.width graphemes)
}

impl WindowLineProperties {
    pub fn lines_u16(&self) -> Option<u16> {
        if self.lines > u16::MAX as usize {
            None
        } else {
            Some(self.lines as u16)
        }
    }
}

/// Struct that represents a fim window.
pub struct Window {
    #[doc(hidden)]
    doc: Option<Document>,
    #[doc(hidden)]
    opt: Options,
    #[doc(hidden)]
    first_line: usize, // zero-based index of first line in document on screen
    #[doc(hidden)]
    pos_in_doc: DocPosition, // location of cursor in document
    #[doc(hidden)]
    raw_window_pos: Position, // terminal location of (0, 0) in raw window (i.e. including line numbers as part of window)
    #[doc(hidden)]
    raw_window_size: Size,
    #[doc(hidden)]
    text_start: u16, // window location of first character
    #[doc(hidden)]
    text_width: u16, // length of space allocated for content
    #[doc(hidden)]
    target_x: usize, // target x-value (used for moving up and down in documents)
    #[doc(hidden)]
    line_properties: Vec<WindowLineProperties>,
}

impl Window {
    /// Create a new, full-terminal Window with the default welcome message.
    pub fn default(term: &Terminal, opt: Options) -> Self {
        let size = term.size();
        assert!(size.height > 1 && size.width > 1);
        let size = Size{ width: size.width, height: size.height - 1 };
        Window{ doc: None, first_line: 0, pos_in_doc: DocPosition::default(), raw_window_pos: Position::default(), raw_window_size: size, text_start: 0, text_width: size.width - 1, target_x: 0, opt, line_properties: Vec::new() }
    }

    /// Create a new, full-terminal Window with the contents of the given file.
    pub fn new(filename: PathBuf, term: &Terminal, opt: Options) -> Result<Self> {
        let size = term.size();
        assert!(size.height > 1 && size.width > 1);
        let size = Size{ width: size.width, height: size.height - 1 };
        let pos_in_doc = DocPosition::default();
        let document = Document::new(filename)?;
        let (text_start, text_width) = Self::compute_text_attrs(&opt, &size, document.num_lines());
        let line_properties = Self::setup_line_properties(&document, text_width);
        Ok(Window{ doc: Some(document), first_line: 0, pos_in_doc, raw_window_pos: Position::default(), raw_window_size: size, text_start, text_width, target_x: 0, opt, line_properties })
    }

    /// Update the window's options.
    pub fn update_options(&mut self, opt: &Options) {
        self.opt = opt.clone();
    }

    /// Render this window's contents to the terminal screen.
    pub fn render(&self, term: &mut Terminal) -> Result<()> {
        if self.doc.is_some() {
            self.draw_document(term)
        } else {
            self.draw_welcome_screen(term)
        }
    }

    /// Move the cursor one character left, if possible.
    pub fn move_left(&mut self, term: &mut Terminal) -> Result<()> {
        if self.doc.is_none() { return Ok(()) }
        if self.pos_in_doc.x > 0 {
            self.pos_in_doc.x -= 1;
            self.target_x = self.pos_in_doc.x;
            self.q_move(term)?;
            term.flush()?;
        }
        Ok(())
    }

    /// Move the cursor one character right, if possible.
    pub fn move_right(&mut self, term: &mut Terminal) -> Result<()> {
        if self.doc.is_none() { return Ok(()) }
        if self.pos_in_doc.x + 1 < self.doc.as_ref().unwrap().line(self.pos_in_doc.y).unwrap().text.len() {
            self.pos_in_doc.x += 1;
            self.target_x = self.pos_in_doc.x;
            self.q_move(term)?;
            term.flush()?;
        }
        Ok(())
    }

    /// Move the cursor one line up, if possible.
    ///
    /// If the line the cursor moves to is long enough, the cursor will stay in the same terminal
    /// column.
    pub fn move_up(&mut self, term: &mut Terminal) -> Result<()> {
        if self.doc.is_none() { return Ok(()) }
        if self.pos_in_doc.y > 0 {
            self.pos_in_doc.y -= 1;
            self.pos_in_doc.x = min(self.target_x, self.doc.as_ref().unwrap().line(self.pos_in_doc.y).unwrap().length);

            if self.pos_in_doc.y + 1 == self.first_line {
                self.first_line -= 1;
                self.render(term)?;
            } else if let LineNumbers::Relative = self.opt.line_numbering {
                self.update_line_numbers(term)?;
            }
            self.q_move(term)?;
            term.flush()?;
        }
        Ok(())
    }

    /// Move the cursor one line down, if possible.
    ///
    /// If the line the cursor moves to is long enough, the cursor will stay in the same terminal
    /// column.
    pub fn move_down(&mut self, term: &mut Terminal) -> Result<()> {
        if self.doc.is_none() { return Ok(()) }
        if self.pos_in_doc.y + 1 < self.doc.as_ref().unwrap().num_lines() {
            self.pos_in_doc.y += 1;
            self.pos_in_doc.x = min(self.target_x, self.doc.as_ref().unwrap().line(self.pos_in_doc.y).unwrap().length);

            if self.pos_in_doc.y == self.first_line + self.raw_window_size.height as usize {
                self.first_line += 1;
                self.render(term)?;
            } else if let LineNumbers::Relative = self.opt.line_numbering {
                self.update_line_numbers(term)?;
            }
            self.q_move(term)?;
            term.flush()?;
        }
        Ok(())
    }
    
    // NOTE: when you implement splitting, make sure that all split windows have
    // documents, and that you change the existing window to have a new blank document
    // if it doesn't have a document, so that the invariants for draw_welcome_screen are
    // maintained. (TODO)
    
    /// Convert between window-text coordinates and terminal coordinates.
    ///
    /// For example, (0, 0) in window-text coordinates represents the location of the first
    /// character of content you can see in the window, whereas (0, 0) in terminal coordinates
    /// represents absolute coordinates, well, in the terminal.
    pub fn to_term(&self, x: u16, y: u16) -> Position {
        assert!(x < self.text_width && y < self.raw_window_size.height);
        Position{ x: x + self.raw_window_pos.x + self.text_start, y: y + self.raw_window_pos.y }
    }

    fn raw_to_term(&self, x: u16, y: u16) -> Position {
        assert!(x < self.raw_window_size.width && y < self.raw_window_size.height);
        Position{ x: x + self.raw_window_pos.x, y: y + self.raw_window_pos.y }
    }

    fn to_window_text(&self) -> Option<Position> {
        if self.pos_in_doc.y < self.first_line { return None; }
        let lines_from_line = div_ceil(self.pos_in_doc.x, self.text_width);
        let x = (self.pos_in_doc.x % self.text_width as usize) as u16;
        let mut y = 0;
        for line in self.first_line..self.pos_in_doc.y {
            y += self.line_properties[line].lines
        }
        y += lines_from_line - 1;
        if y >= self.raw_window_size.height.into() || x >= self.text_width { None }
        else { Some(Position{ x, y: y as u16 }) } // y guaranteed to fit into u16 since < height, which is u16
    }

    fn q_move(&self, term: &mut Terminal) -> Result<()> {
        if let Some(Position{ x: x_wt, y: y_wt }) = self.to_window_text() {
            let Position{ x, y } = self.to_term(x_wt, y_wt); 
            term.cursor_to(x, y).q_move_cursor()?;
        }
        Ok(())
    }

    fn setup_line_properties(doc: &Document, text_width: u16) -> Vec<WindowLineProperties> {
        let mut line_properties = Vec::with_capacity(doc.num_lines()); 
        for line in doc {
            let rem = (line.length % text_width as usize) as u16;
            let lines = div_ceil(line.length, text_width);
            let props = WindowLineProperties{ lines, graphemes: rem };
            line_properties.push(props);
        }
        line_properties
    }

    fn window_to_doc(&self, line: u16) -> WindowLineType {
        let mut window_line = 0u16;
        let mut doc_line = self.first_line;
        let line_count = self.doc.as_ref().unwrap().num_lines();
        while window_line < line && doc_line < line_count {
            if let Some(l) = self.line_properties[doc_line].lines_u16() {
                window_line += l;
                doc_line += 1;
            } else {
                return WindowLineType::WrappedLine;
            }
        }
        if doc_line >= line_count {
            WindowLineType::Tilde
        } else if window_line == line {
            WindowLineType::DocLine(doc_line)
        } else {
            WindowLineType::WrappedLine
        }
    }

    fn line_number(&self, line: u16) -> StyledContent<String> {
        match self.window_to_doc(line) {
            WindowLineType::WrappedLine => "".to_string().stylize(),
            WindowLineType::Tilde => "~".to_string().blue(),
            WindowLineType::DocLine(doc_line) => {
                // if line as usize >= self.doc.as_ref().map(|d| d.num_lines()).or(Some(usize::MAX)).unwrap() { return "~".to_string().blue() }
                match self.opt.line_numbering {
                    LineNumbers::Off => String::new(),
                    LineNumbers::On => once(' ').chain((doc_line + 1).to_string().chars().rev())
                                                .chain(repeat(' ')).take(self.text_start as usize).collect::<String>()
                                                .chars().rev().collect::<String>(),
                    LineNumbers::Relative => {
                        if self.pos_in_doc.y == doc_line as usize {
                            (doc_line + 1).to_string().chars().chain(repeat(' ')).take(self.text_start as usize).collect::<String>()
                        } else {
                            once(' ').chain(abs_diff(self.pos_in_doc.y, doc_line).to_string().chars().rev())
                                     .chain(repeat(' ')).take(self.text_start as usize).collect::<String>()
                                     .chars().rev().collect::<String>()
                        }
                    }
                }.dark_yellow()
            }
        }
    }

    /*fn update_line_numbers(&self, term: &mut Terminal) -> Result<()> {
        if let LineNumbers::Off = self.opt.line_numbering { return Ok(()); }
        term.q(Hide)?.save_cursor();
        self.q_clear(ClearType::LineNumbers, term)?;
        for line in 0..self.raw_window_size.height {
            let Position{ x, y } = self.raw_to_term(0, line);
            term.cursor_to(x, y).q_move_cursor()?.q(Print(self.line_number(line)))?;
        }
        term.restore_cursor();
        term.q_move_cursor()?.q(Show)?.flush()?;
        Ok(())
    }*/

    fn update_line_numbers(&self, term: &mut Terminal) -> Result<()> {
        if let LineNumbers::Off = self.opt.line_numbering { return Ok(()); }
        term.q(Hide)?.save_cursor();
        self.q_clear(ClearType::LineNumbers, term)?;
        let mut window_line: u16 = 0;
        let mut doc_line: usize = self.first_line;
        let line_count = self.doc.as_ref().unwrap().num_lines();
        while window_line < self.raw_window_size.height && doc_line < line_count {
            let Position{ x, y } = self.raw_to_term(0, window_line);
            term.cursor_to(x, y).q_move_cursor()?.q(Print(self.line_number(window_line)))?;
            let end = min(self.line_properties[doc_line].lines_u16().unwrap_or(u16::MAX), self.raw_window_size.height - window_line); 
            window_line += end;
            doc_line += 1;
        }
        while window_line < self.raw_window_size.height {
            let Position{ x, y } = self.raw_to_term(0, window_line);
            term.cursor_to(x, y).q_move_cursor()?.q(Print(self.line_number(window_line)))?;
            window_line += 1;
        }
        term.restore_cursor();
        term.q_move_cursor()?.q(Show)?.flush()
    }

    fn q_clear(&self, clear_type: ClearType, term: &mut Terminal) -> Result<()> {
        // does not change cursor visibility
        let clear_line = " ".repeat(self.raw_window_size.width as usize);
        let clear_line_numbers = " ".repeat(self.text_start as usize);
        term.save_cursor();
        for line in 0..self.raw_window_size.height {
            let Position{ x, y } = self.raw_to_term(0, line);
            term.cursor_to(x, y).q_move_cursor()?.q(Print(match clear_type {
                ClearType::All => &clear_line,
                ClearType::LineNumbers => &clear_line_numbers,
            }))?;
        }
        term.restore_cursor();
        term.q_move_cursor()?;
        Ok(())
    }
    
    fn compute_text_attrs(opt: &Options, raw_window_size: &Size, doc_length: usize) -> (u16, u16) {
        // includes extra space after line numbers
        let line_number_chars: u16 = match opt.line_numbering {
            LineNumbers::Off => 0,
            _ => max(log10(doc_length) + 1, 3),
        };
        let text_width = saturating_sub(raw_window_size.width, line_number_chars);
        (line_number_chars, text_width)
    }

    fn draw_document(&self, term: &mut Terminal) -> Result<()> {
        if let Some(doc) = self.doc.as_ref() {
            #[derive(Copy, Clone)]
            enum LineType<'a> {
                Content(&'a str),
                Continued(&'a str),
                Tilde
            }

            let text_width = self.text_width as usize;

            term.q(Hide)?.save_cursor();
            self.q_clear(ClearType::All, term)?;
            doc.iter_from(self.first_line).unwrap() // we assert that first_line is a valid index
               .flat_map(|l| {
                    let mut graphemes = l.text.as_str().grapheme_indices(true);
                    let output: Box<dyn Iterator<Item = LineType>> = if let Some((idx, _)) = graphemes.nth(text_width + 1) {
                        let mut indices = vec![idx]; 
                        // note that we've already consumed the first character of the next chunk
                        // we need to get the (width + 1)th character from the start of the chunk,
                        // width + 1 - 1 = width
                        while let Some((idx, _)) = graphemes.nth(self.raw_window_size.width.into()) { // TODO: check logic here!
                            indices.push(idx);
                        }
                        let mut pieces: Vec<&str> = Vec::new();
                        let first = if indices.len() > 1 { &l.text[0..indices[1]] } else { l.text.as_str() };
                        for i in 1..indices.len() {
                            pieces.push(&l.text[indices[i - 1]..indices[i]]);
                        }
                        Box::new(once(LineType::Content(first)).chain(pieces.into_iter().map(|p| LineType::Continued(p))))
                    } else {
                        Box::new(once(LineType::Content(l.text.as_str())))
                    };
                    output
                })
               .chain(repeat(LineType::Tilde))
               .enumerate()
               .take(self.raw_window_size.height.into())
               .try_for_each(|(terminal_line, lt)| { 
                    term.cursor_to(0, terminal_line as u16).q_move_cursor()?;
                    if let LineType::Content(text) = lt {
                        term.q(Print(self.line_number(terminal_line as u16)))?.q(Print(text))
                    } else if let LineType::Continued(text) = lt {
                        term.q(Print(" ".repeat(self.text_start as usize)))?.q(Print(text)) // TODO: check logic here
                    } else {
                        term.q(Print("~".blue()))
                    }.map(|_| ())
                })?;
            term.q(Show)?.restore_cursor();
            term.q_move_cursor()?.flush()
        } else {
            Ok(())
        }
    }

    fn center_welcome(&self, idx: usize, term: &mut Terminal) -> Result<()> {
        let line = &WELCOME_MSG[idx];
        let width = self.raw_window_size.width as usize;
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
        let message_begin_line = if self.raw_window_size.height / 2 < message_len / 2 { 0 } else {
            self.raw_window_size.height / 2 - message_len / 2
        };
        let mut message_line: u16 = 0;
        term.q(Hide)?.save_cursor();
        self.q_clear(ClearType::All, term)?;
        for i in 0..self.raw_window_size.height {
            let Position{ x, y } = self.raw_to_term(0, i);
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

fn div_ceil(quotient: usize, divisor: u16) -> usize {
    if quotient == 0 { 1 } else { (quotient as f64 / divisor as f64).ceil() as usize }
}

fn abs_diff(x: usize, y: usize) -> usize {
    if x > y { x - y } else { y - x }
}

fn saturating_sub(x: u16, y: u16) -> u16 {
    if y >= x {
        0
    } else {
        x - y
    }
}

fn log10(mut x: usize) -> u16 {
    let mut log = 0u16;
    while x > 0 {
        x /= 10;
        log += 1;
    }
    log
}
