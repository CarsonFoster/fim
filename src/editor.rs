use std::io::{Write, Stdout, stdout};
use crossterm::{
    Result,
    execute,
    terminal::{
        self,
        EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};

pub struct Editor {
    stdout: Stdout,
}

impl Editor {
    pub fn new() -> Editor {
        Editor{ stdout: stdout() }
    }

    pub fn run(&mut self) -> Result<()> {
        self.setup()?;
        println!("Hello, world!\r");
        loop {

        }
        Ok(())
    }

    fn setup(&mut self) -> Result<()> {
        execute!(self.stdout, EnterAlternateScreen)?;
        terminal::enable_raw_mode()
    }
}

impl Drop for Editor {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Failed to disable raw mode.");
        execute!(self.stdout, LeaveAlternateScreen).expect("Failed to leave alternate screen.");
    }
}
