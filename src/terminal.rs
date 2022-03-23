use std::io::{Stdout, stdout};
use crossterm::{
    Result,
    cursor::{
        Hide,
        MoveTo,
        Show,
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
        Clear,
        ClearType,
        LeaveAlternateScreen,
    },
};

pub struct Size {
    pub width: u16,
    pub height: u16,
}

pub struct Position {
    pub x: u16,
    pub y: u16,
}

pub struct Terminal {
    size: Size,
    stdout: Stdout,
    cursor_pos: Position,
}

impl Terminal {
    pub fn new() -> Result<Self> {
        let (width, height) = terminal::size()?;
        let stdout = stdout();
        terminal::enable_raw_mode()?;
        Ok( Terminal{ stdout, size: Size{ width, height }, cursor_pos: Position{ x: 0, y: 0 } } )
    }

    fn saturating_sub(x: u16, d: u16) -> u16 {
        if d > x {
            0
        } else {
            x - d
        }
    }

    fn set_x(&mut self, x: u16) {
        if x >= 0 && x < self.size.width {
            self.cursor_pos.x = x;
        }
    }

    fn set_y(&mut self, y: u16) {
        if y >= 0 && y < self.size.height {
            self.cursor_pos.y = y;
        }
    }

    pub fn size(&self) -> &Size {
        &self.size
    }

    pub fn cursor_pos(&self) -> &Position {
        &self.cursor_pos
    }

    pub fn hide_cursor(&mut self) -> Result<()> {
        execute!(self.stdout, Hide)
    }

    pub fn show_cursor(&mut self) -> Result<()> {
        execute!(self.stdout, Show)
    }

    pub fn move_cursor(&mut self) -> Result<()> { 
        execute!(self.stdout, MoveTo(self.cursor_pos.x, self.cursor_pos.y))
    }

    pub fn move_cursor_to(&mut self, x: u16, y: u16) -> Result<()> {
        self.cursor_to(x, y);
        self.move_cursor()
    }

    pub fn cursor_to(&mut self, x: u16, y: u16) {
        self.set_x(x);
        self.set_y(y);
    }

    pub fn cursor_left_by(&mut self, d_x: u16) {
        self.cursor_to(Self::saturating_sub(self.cursor_pos.x, d_x), self.cursor_pos.y);
    }

    pub fn cursor_right_by(&mut self, d_x: u16) {
        self.cursor_to(self.cursor_pos.x.saturating_add(d_x), self.cursor_pos.y);
    }

    pub fn cursor_up_by(&mut self, d_y: u16) {
        self.cursor_to(self.cursor_pos.x, Self::saturating_sub(self.cursor_pos.y, d_y));
    }

    pub fn cursor_down_by(&mut self, d_y: u16) {
        self.cursor_to(self.cursor_pos.x, self.cursor_pos.y.saturating_add(d_y));
    }

    pub fn enter_alternate_screen(&mut self) -> Result<()> {
        execute!(self.stdout, EnterAlternateScreen)
    }

    pub fn leave_alternate_screen(&mut self) -> Result<()> {
        execute!(self.stdout, LeaveAlternateScreen)
    }

    pub fn clear_all(&mut self) -> Result<()> {
        execute!(self.stdout, Clear(ClearType::All))
    }

    pub fn read_key(&self) -> Result<KeyEvent> {
        loop {
            let event = read()?;
            if let Event::Key(key_event) = event {
                return Ok(key_event);
            }
        }
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Failed to disable raw mode.");
    }
}

