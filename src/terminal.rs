use std::io::{Stdout, stdout};
use crossterm::{
    Result,
    execute,
    terminal::{
        self,
        EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};

pub struct Size {
    pub width: u16,
    pub height: u16,
}

pub struct Terminal {
    size: Size,
    stdout: Stdout,
}

impl Terminal {
    pub fn new() -> Result<Self> {
        let (width, height) = terminal::size()?;
        let stdout = stdout();
        terminal::enable_raw_mode()?;
        Ok( Terminal{ stdout, size: Size{ width, height } } )
    }

    pub fn size(&self) -> &Size {
        &self.size
    }

    pub fn enter_alternate_screen(&mut self) -> Result<()> {
        execute!(self.stdout, EnterAlternateScreen)
    }

    pub fn leave_alternate_screen(&mut self) -> Result<()> {
        execute!(self.stdout, LeaveAlternateScreen)
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Failed to disable raw mode.");
    }
}

