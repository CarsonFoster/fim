//! A module for dealing with the terminal device.
use std::io::{Stdout, Write, stdout};
use crossterm::{
    Command,
    Result,
    QueueableCommand,
    cursor::{
        MoveTo,
    },
    event::{
        Event,
        KeyEvent,
        read,
    },
    execute,
    style::{
        ContentStyle,
        Print,
    },
    terminal::{
        self,
        EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};

/// Struct that represents a 2D terminal size.
#[derive(Copy, Clone)]
pub struct Size {
    pub width: u16,
    pub height: u16,
}

/// Struct that represents a 2D position on the terminal.
#[derive(Copy, Clone)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}

/// Struct that represents styled, centered content to display.
pub struct Centered<'a> {
    #[doc(hidden)]
    stdout: &'a mut Stdout,
    #[doc(hidden)]
    term_length: usize,
    #[doc(hidden)]
    prefix: &'a str,
    #[doc(hidden)]
    text: &'a str,
    #[doc(hidden)]
    suffix: &'a str,
    #[doc(hidden)]
    prefix_styles: Option<ContentStyle>,
    #[doc(hidden)]
    text_styles: Option<ContentStyle>,
    #[doc(hidden)]
    suffix_styles: Option<ContentStyle>,
}

impl<'a> Centered<'a> {
    /// Create a new Centered struct.
    ///
    /// The 'prefix' is printed at the beginning of the line, the 'text' is printed in the center
    /// of the line, and the 'suffix' is printed at the end of the line.
    /// Priority is given to the 'text' if not everything fits.
    pub fn new(stdout: &'a mut Stdout, term_length: usize, prefix: &'a str, text: &'a str, suffix: &'a str,
               prefix_styles: Option<ContentStyle>, text_styles: Option<ContentStyle>, suffix_styles: Option<ContentStyle>) -> Self {
        Centered{ stdout, term_length, prefix, text, suffix, prefix_styles, text_styles, suffix_styles }
    }

    /// Queue the relevant
    /// [`Command`](https://docs.rs/crossterm/latest/crossterm/trait.Command.html)s to print the styled content.
    pub fn q(&mut self) -> Result<()> {
        let length = self.text.len();
        let prefix_length = self.prefix.len();
        let suffix_length = self.suffix.len();
        let no_style = ContentStyle::new();

        if length + prefix_length + suffix_length < self.term_length {
            // everything fits, add padding; extra padding goes on the end
            let padding = self.term_length - length - prefix_length - suffix_length;
            let left_padding = self.term_length / 2 - length / 2 - prefix_length;
            let right_padding = padding - left_padding;
            self.stdout.queue(Print(self.prefix_styles.map_or_else(|| no_style.apply(self.prefix), |s| s.apply(self.prefix))))?;
            self.stdout.queue(Print(" ".repeat(left_padding)))?;
            self.stdout.queue(Print(self.text_styles.map_or_else(|| no_style.apply(self.text), |s| s.apply(self.text))))?;
            self.stdout.queue(Print(" ".repeat(right_padding)))?;
            self.stdout.queue(Print(self.suffix_styles.map_or_else(|| no_style.apply(self.suffix), |s| s.apply(self.suffix))))?;
        } else if length < self.term_length {
            // at least the main self.text fits
            // adds as many self.prefix and self.suffix characters as possible; if self.prefix/self.suffix runs out,
            // padding is used for the remaining characters instead
            // extra character goes on left
            let remainder = self.term_length - length;
            let right_length = remainder / 2;
            let left_length = remainder - right_length;
            if self.prefix.len() <= left_length {
                self.stdout.queue(Print(self.prefix_styles.map_or_else(|| no_style.apply(self.prefix), |s| s.apply(self.prefix))))?;
                self.stdout.queue(Print(" ".repeat(left_length - self.prefix.len())))?;
            } else {
                let string = &self.prefix[0..left_length];
                self.stdout.queue(Print(self.prefix_styles.map_or_else(|| no_style.apply(string), |s| s.apply(string))))?;
            }
            self.stdout.queue(Print(self.text_styles.map_or_else(|| no_style.apply(self.text), |s| s.apply(self.text))))?;
            if self.suffix.len() <= right_length {
                self.stdout.queue(Print(" ".repeat(right_length - self.suffix.len())))?;
                self.stdout.queue(Print(self.suffix_styles.map_or_else(|| no_style.apply(self.suffix), |s| s.apply(self.suffix))))?;
            } else {
                let string = &self.suffix[0..right_length];
                self.stdout.queue(Print(self.suffix_styles.map_or_else(|| no_style.apply(string), |s| s.apply(string))))?;
            }
        } else {
            // main text doesn't fit, include self.term_length characters from the middle
            // bias toward extra character on the beginning
            let removed_chars = length - self.term_length;
            let skip = removed_chars / 2;
            let bytes: Vec<u8> = self.text.bytes().skip(skip).take(self.term_length).collect(); 
            let string = String::from_utf8(bytes).expect("centered expects only ASCII!");
            let slice: &str = string.as_ref();
            self.stdout.queue(Print(self.text_styles.map_or_else(|| no_style.apply(slice), |s| s.apply(slice))))?;
        }
        Ok(())
    }
}

/// Struct to represent the actual terminal the program is displayed in.
pub struct Terminal {
    #[doc(hidden)]
    size: Size,
    #[doc(hidden)]
    stdout: Stdout,
    #[doc(hidden)]
    cursor_pos: Position,
    #[doc(hidden)]
    cursor_stack: Vec<Position>,
}

impl Terminal {
    /// Create a new Terminal struct.
    pub fn new() -> Result<Self> {
        let (width, height) = terminal::size()?;
        let stdout = stdout();
        terminal::enable_raw_mode()?;
        Ok( Terminal{ stdout, size: Size{ width, height }, cursor_pos: Position{ x: 0, y: 0 }, cursor_stack: Vec::new() } )
    }

    fn saturating_sub(x: u16, d: u16) -> u16 {
        if d > x {
            0
        } else {
            x - d
        }
    }

    fn set_x(&mut self, x: u16) {
        if x < self.size.width {
            self.cursor_pos.x = x;
        }
    }

    fn set_y(&mut self, y: u16) {
        if y < self.size.height {
            self.cursor_pos.y = y;
        }
    }

    /// Return a copy of this terminal's size.
    pub fn size(&self) -> Size {
        self.size
    }

    /// Return a reference to this terminal's current cursor position.
    pub fn cursor_pos(&self) -> &Position {
        &self.cursor_pos
    }

    /// Save the position of the cursor.
    pub fn save_cursor(&mut self) {
        self.cursor_stack.push(self.cursor_pos);
    }

    /// Restore the position of the cursor.
    pub fn restore_cursor(&mut self) {
        if let Some(pos) = self.cursor_stack.pop() {
            self.cursor_pos = pos;
        }
    }

    /// Move the cursor immediately.
    ///
    /// This executes, instead of queues, a
    /// [`Command`](https://docs.rs/crossterm/latest/crossterm/trait.Command.html). See
    /// [here](https://docs.rs/crossterm/latest/crossterm/index.html#command-api).
    pub fn move_cursor(&mut self) -> Result<()> { 
        execute!(self.stdout, MoveTo(self.cursor_pos.x, self.cursor_pos.y))
    }

    /// Queues the cursor move.
    pub fn q_move_cursor(&mut self) -> Result<&mut Self> {
        self.stdout.queue(MoveTo(self.cursor_pos.x, self.cursor_pos.y))?;
        Ok(self)
    }

    /// Move the cursor to a location immediately.
    ///
    /// This executes, instead of queues, a [`Command`](https://docs.rs/crossterm/latest/crossterm/trait.Command.html). See
    /// [here](https://docs.rs/crossterm/latest/crossterm/index.html#command-api).
    pub fn move_cursor_to(&mut self, x: u16, y: u16) -> Result<()> {
        self.cursor_to(x, y);
        self.move_cursor()
    }

    /// Set the cursor position to a location.
    ///
    /// This does not queue or execute any [`Command`](https://docs.rs/crossterm/latest/crossterm/trait.Command.html)s.
    pub fn cursor_to(&mut self, x: u16, y: u16) -> &mut Self {
        self.set_x(x);
        self.set_y(y);
        self
    }

    /// Move the cursor left some amount, if able.
    ///
    /// This does not queue or execute any [`Command`](https://docs.rs/crossterm/latest/crossterm/trait.Command.html)s.
    pub fn cursor_left_by(&mut self, d_x: u16) -> &mut Self {
        self.cursor_to(Self::saturating_sub(self.cursor_pos.x, d_x), self.cursor_pos.y);
        self
    }

    /// Move the cursor right some amount, if able.
    ///
    /// This does not queue or execute any [`Command`](https://docs.rs/crossterm/latest/crossterm/trait.Command.html)s.
    pub fn cursor_right_by(&mut self, d_x: u16) -> &mut Self {
        self.cursor_to(self.cursor_pos.x.saturating_add(d_x), self.cursor_pos.y);
        self
    }

    /// Move the cursor up some amount, if able.
    ///
    /// This does not queue or execute any [`Command`](https://docs.rs/crossterm/latest/crossterm/trait.Command.html)s.
    pub fn cursor_up_by(&mut self, d_y: u16) -> &mut Self {
        self.cursor_to(self.cursor_pos.x, Self::saturating_sub(self.cursor_pos.y, d_y));
        self
    }

    /// Move the cursor down some amount, if able.
    ///
    /// This does not queue or execute any [`Command`](https://docs.rs/crossterm/latest/crossterm/trait.Command.html)s.
    pub fn cursor_down_by(&mut self, d_y: u16) -> &mut Self {
        self.cursor_to(self.cursor_pos.x, self.cursor_pos.y.saturating_add(d_y));
        self
    }

    /// Enter the alternate screen.
    pub fn enter_alternate_screen(&mut self) -> Result<()> {
        execute!(self.stdout, EnterAlternateScreen)
    }

    /// Exit the alternate screen.
    pub fn leave_alternate_screen(&mut self) -> Result<()> {
        execute!(self.stdout, LeaveAlternateScreen)
    }

    /// Queue a [`Command`](https://docs.rs/crossterm/latest/crossterm/trait.Command.html).
    pub fn q(&mut self, cmd: impl Command) -> Result<&mut Self> {
        self.stdout.queue(cmd)?;
        Ok(self)
    }

    /// Flush the queued commands to standard output.
    pub fn flush(&mut self) -> Result<()> {
        self.stdout.flush()
    }

    /// Poll a [`KeyEvent`](https://docs.rs/crossterm/latest/crossterm/event/struct.KeyEvent.html) (blocking).
    pub fn read_key(&self) -> Result<KeyEvent> {
        loop {
            let event = read()?;
            if let Event::Key(key_event) = event {
                return Ok(key_event);
            }
        }
    }

    /// Creates a new [`Centered`] struct with the current terminal's standard output handle.
    pub fn centered_styles<'a>(&'a mut self, prefix: &'a str, text: &'a str, suffix: &'a str,
                      prefix_styles: Option<ContentStyle>, text_styles: Option<ContentStyle>, suffix_styles: Option<ContentStyle>) -> Centered<'a> {
        Centered::new(&mut self.stdout, self.size.width as usize, prefix, text, suffix, prefix_styles, text_styles, suffix_styles)  
    }

    /// Centers text (no styles).
    ///
    /// See also: [`Centered`]
    pub fn centered(&self, prefix: &str, text: &str, suffix: &str) -> String {
        // NOTE: does not deal with graphemes/unicode! ASCII only
        // nothing should overflow; a usize amount of text is an insane amount
        let length = text.len();
        let term_length = self.size.width as usize;
        let prefix_length = prefix.len();
        let suffix_length = suffix.len();

        let mut result = String::with_capacity(term_length);
        if length + prefix_length + suffix_length < term_length {
            // everything fits, add padding; extra padding goes on the end
            let padding = term_length - length - prefix_length - suffix_length;
            let left_padding = term_length / 2 - length / 2 - prefix_length;
            let right_padding = padding - left_padding;
            result.push_str(prefix);
            for _ in 0..left_padding {
                result.push_str(" ");
            }
            result.push_str(text);
            for _ in 0..right_padding {
                result.push_str(" ");
            }
            result.push_str(suffix);
        } else if length < term_length {
            // at least the main text fits
            // adds as many prefix and suffix characters as possible; if prefix/suffix runs out,
            // padding is used for the remaining characters instead
            // extra character goes on left
            let remainder = term_length - length;
            let right_length = remainder / 2;
            let left_length = remainder - right_length;
            if prefix.len() <= left_length {
                result.push_str(prefix);
                for _ in 0..(left_length - prefix.len()) {
                    result.push_str(" ");
                }
            } else {
                result.push_str(&prefix[0..left_length]);
            }
            result.push_str(text);
            if suffix.len() <= right_length {
                for _ in 0..(right_length - suffix.len()) {
                    result.push_str(" ");
                }
                result.push_str(suffix);
            } else {
                result.push_str(&suffix[0..right_length]);
            }
        } else {
            // main text doesn't fit, include term_length characters from the middle
            // bias toward extra character on the beginning
            let removed_chars = length - term_length;
            let skip = removed_chars / 2;
            let bytes: Vec<u8> = text.bytes().skip(skip).take(term_length).collect(); 
            result = String::from_utf8(bytes).expect("centered expects only ASCII!");
        }
        result
    }
}

#[doc(hidden)]
impl Drop for Terminal {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Failed to disable raw mode.");
    }
}
