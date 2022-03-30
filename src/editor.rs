use crate::terminal::Terminal;
use crossterm::{
    Result,
    cursor,
    event::{
        KeyCode,
        KeyEvent,
    },
    terminal::{
        Clear,
        ClearType,
    },
    style::Print,
};

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Editor {
    terminal: Terminal,
    quit: bool,
    welcome_message: [String; 4],
}

impl Editor {
    pub fn new() -> Result<Editor> {
        let welcome_message = ["FIM - Foster's vi IMproved".into(), String::new(), format!("Version {}", VERSION), "by Carson Foster".into()];
        Ok( Editor{ terminal: Terminal::new()?, quit: false, welcome_message } )
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
        self.draw_welcome_screen()
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
            _ => &mut self.terminal // the other functions return &mut Terminal so you can chain them
        };
        self.terminal.move_cursor()
    }

    fn draw_welcome_screen(&mut self) -> Result<()> {
        let height = self.terminal.size().height;
        let mut message_line = 0;
        self.terminal.q(cursor::SavePosition)?.q(cursor::Hide)?.q(Clear(ClearType::All))?;
        for i in 0..(height - 1) {
            if i == height / 2 + message_line {
                self.terminal.q(Print(self.terminal.centered("~", &self.welcome_message[message_line as usize], "") + "\r\n"));
                message_line += 1;
            } else {
                self.terminal.q(Print("~\r\n"))?;
            }
        }
        self.terminal.q(Print("~"))?.q(cursor::RestorePosition)?.q(cursor::Show)?.flush()
    }
}

impl Drop for Editor {
    fn drop(&mut self) {
        self.terminal.leave_alternate_screen().expect("Failed to leave alternate screen");
    }
}
