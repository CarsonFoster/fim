use crossterm::{Result, terminal};

pub struct Editor;

impl Editor {
    pub fn new() -> Editor {
        Editor
    }

    pub fn run(&self) -> Result<()> {
        self.setup()?;
        println!("Hello, world!\r");
        Ok(())
    }

    fn setup(&self) -> Result<()> {
        terminal::enable_raw_mode() 
    }
}

impl Drop for Editor {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Failed to disable raw mode.");
    }
}
