use crate::context::*;
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
    style::{
        ContentStyle,
        Print,
        Stylize,
    },
};

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Editor<'a> {
    terminal: Terminal,
    quit: bool,
    context_stack: Vec<Box<dyn Context + 'a>>,
    welcome_message: [String; 4],
}

impl<'a> Editor<'a> {
    pub fn new() -> Result<Editor<'a>> {
        let welcome_message = ["FIM - Foster's vIM-like editor".into(), String::new(), format!("Version {}", VERSION), "by Carson Foster".into()];
        Ok( Editor{ terminal: Terminal::new()?, quit: false, welcome_message, context_stack: vec![Box::new(NormalMode)] } )
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
        let event = self.terminal.read_key()?;
        if let Some(mut last_box) = self.context_stack.pop() {
            let mut msg = last_box.forward(self, event)?;
            while msg.is_some() && self.context_stack.len() > 0 {
                last_box = self.context_stack.pop().unwrap();
                msg = last_box.receive(self, msg.unwrap())?;
            }
            self.context_stack.push(last_box);
        }

//        let KeyEvent{ code: c, modifiers: m } = self.terminal.read_key()?;
//        match c {
//            KeyCode::Char('q') => self.quit = true,
//            KeyCode::Char('h') | KeyCode::Char('j') | KeyCode::Char('k') | KeyCode::Char('l') => self.move_key(c)?,
//            _ => ()
//        }
        Ok(())
    }

    pub fn quit(&mut self) {
        self.quit = true;
    }

    pub fn push_context<C: 'a + Context>(&mut self, context: C) {
        self.context_stack.push(Box::new(context)); 
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
        let message_len = self.welcome_message.len() as u16;
        let mut message_line: u16 = 0;
        self.terminal.q(cursor::SavePosition)?.q(cursor::Hide)?.q(Clear(ClearType::All))?;
        for i in 0..(height - 1) {
            if message_line < message_len && i == height / 2 - message_len / 2 + message_line {
                self.terminal.centered_styles("~", &self.welcome_message[message_line as usize], "",
                                              Some(ContentStyle::new().blue()), None, None).q()?;
                // self.terminal.q(Print(self.terminal.centered("~", &self.welcome_message[message_line as usize], "") + "\r\n"));
                message_line += 1;
            } else {
                self.terminal.q(Print("~\r\n".blue()))?;
            }
        }
        self.terminal.q(Print("~".blue()))?.q(cursor::RestorePosition)?.q(cursor::Show)?.flush()
    }
}

impl<'a> Drop for Editor<'a> {
    fn drop(&mut self) {
        self.terminal.leave_alternate_screen().expect("Failed to leave alternate screen");
    }
}
