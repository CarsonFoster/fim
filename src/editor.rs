use crate::terminal::Terminal;
use crossterm::{
    event::{
        KeyCode,
        KeyEvent,
    },
    Result,
};

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
        loop {
            self.process_keypress()?; 
            if self.quit {
                break;
            }
        }
        Ok(())
    }

    fn setup(&mut self) -> Result<()> {
        self.terminal.enter_alternate_screen()?;
        self.terminal.move_cursor_to(0, 0)?;
        self.draw_welcome_screen();
        Ok(())
    }

    fn process_keypress(&mut self) -> Result<()> {
        let KeyEvent{ code: c, modifiers: m } = self.terminal.read_key()?;
        match c {
            KeyCode::Char('q') => self.quit = true,
            KeyCode::Char('h') | KeyCode::Char('j') | KeyCode::Char('k') | KeyCode::Char('l') => self.move_key(c)?,
            _ => ()
        }
        Ok(())
    }

    fn move_key(&mut self, key: KeyCode) -> Result<()> {
        match key {
            KeyCode::Char('h') => self.terminal.cursor_left_by(1),
            KeyCode::Char('j') => self.terminal.cursor_down_by(1),
            KeyCode::Char('k') => self.terminal.cursor_up_by(1),
            KeyCode::Char('l') => self.terminal.cursor_right_by(1),
            _ => ()
        }
        self.terminal.move_cursor()
    }

    fn draw_welcome_screen(&mut self) -> Result<()> {
        self.terminal.hide_cursor()?;
        self.terminal.clear_all()?;
        for x in 0..(self.terminal.size().height - 1) {
            println!("~\r"); 
        }
        print!("~");
        self.terminal.move_cursor()?;
        self.terminal.show_cursor()
    }
}

impl Drop for Editor {
    fn drop(&mut self) {
        self.terminal.leave_alternate_screen().expect("Failed to leave alternate screen");
    }
}
