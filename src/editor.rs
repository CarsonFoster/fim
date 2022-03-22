use crate::terminal::Terminal;
use crossterm::Result;

pub struct Editor {
    terminal: Terminal,
    quit: bool,
}

impl Editor {
    pub fn new() -> Result<Editor> {
        Ok( Editor{ terminal: Terminal::new()?, quit: false } )
    }

    pub fn run(&mut self) -> Result<()> {
        self.setup()?;
        println!("Hello, world!\r");
        loop {
            self.process_keypress()?; 
            if self.quit {
                break;
            }
        }
        Ok(())
    }

    fn setup(&mut self) -> Result<()> {
        self.terminal.enter_alternate_screen()
    }

    fn process_keypress(&mut self) -> Result<()> {
        Ok(())
    }
}

impl Drop for Editor {
    fn drop(&mut self) {
        self.terminal.leave_alternate_screen().expect("Failed to leave alternate screen");
    }
}
