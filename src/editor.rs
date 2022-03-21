use std::io::{Write, Stdout, stdout};
use crossterm::{
    Result,
    terminal::{
        self,
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
        Ok(())
    }

    fn setup(&mut self) -> Result<()> {
        terminal::enable_raw_mode()
    }
}

impl Drop for Editor {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Failed to disable raw mode.");
    }
}
