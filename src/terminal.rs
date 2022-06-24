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
    terminal::{
        self,
        EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};

/// Struct that represents a 2D terminal size.
#[derive(Copy, Clone)]
pub struct Size {
    /// Length, horizontally.
    pub width: u16,
    /// Length, vertically.
    pub height: u16,
}

/// Struct that represents a 2D position on the terminal.
#[derive(Copy, Clone, Default)]
pub struct Position {
    /// Position, horizontally (this is actually a column number).
    pub x: u16,
    /// Position, vertically (this is actually a row number).
    pub y: u16,
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
}

#[doc(hidden)]
impl Drop for Terminal {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Failed to disable raw mode.");
    }
}
