//! A module that contains the main editor logic.
use crate::context::*;
use crate::terminal::Terminal;
use crate::window::Window;
use crossterm::{
    Result,
    cursor,
    event::KeyCode,
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
const WELCOME_SIZE: usize = 4;
lazy_static! {
    static ref WELCOME_MSG: [String; WELCOME_SIZE] = ["FIM - Foster's vIM-like editor".into(), String::new(), format!("Version {}", VERSION), "by Carson Foster".into()];
}

/// Struct that represents the fim editor.
pub struct Editor<'a> {
    #[doc(hidden)]
    terminal: Terminal,
    #[doc(hidden)]
    quit: bool,
    #[doc(hidden)]
    context_stack: Vec<Box<dyn Context + 'a>>,
    #[doc(hidden)]
    push_context_stack: Vec<Box<dyn Context + 'a>>,
    #[doc(hidden)]
    has_been_setup_stack: Vec<bool>,
    #[doc(hidden)]
    command_stack: Vec<String>,
    #[doc(hidden)]
    windows: Vec<Window>,
}

impl<'a> Editor<'a> {
    /// Create a new Editor struct from a file.
    pub fn new(filename: &str) -> Result<Editor<'a>> {
        let term = Terminal::new()?;
        let window = Window::new(filename, &term)?;
        Ok( Editor{ terminal: term, quit: false, context_stack: vec![Box::new(NormalMode)], push_context_stack: Vec::new(), has_been_setup_stack: vec![true], command_stack: Vec::new(), windows: vec![window] } )
    }

    /// Create a new Editor struct with the default welcome screen.
    pub fn default() -> Result<Editor<'a>> {
        let term = Terminal::new()?;
        let window = Window::default(&term);
        Ok( Editor{ terminal: term, quit: false, context_stack: vec![Box::new(NormalMode)], push_context_stack: Vec::new(), has_been_setup_stack: vec![true], command_stack: Vec::new(), windows: vec![window] } )
    }

    /// Run the editor logic.
    ///
    /// Returns only when the user has signalled they want to quit.
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
        self.windows.iter().try_for_each(|w| w.render(&mut self.terminal))?;
        Ok(())
    }

    fn process_keypress(&mut self) -> Result<()> {
        let event = self.terminal.read_key()?;
        if let Some(mut context) = self.context_stack.pop() {
            self.has_been_setup_stack.pop().unwrap();
            let mut setup;
            let mut msg = context.forward(self, event)?;
            while msg.is_some() {
                context = self.context_stack.pop().expect("Context stack is empty during message propagation");
                setup = self.has_been_setup_stack.pop().unwrap();
                if !setup {
                    let returned = context.setup(self)?;
                    if returned { continue; }
                }
                msg = context.receive(self, msg.unwrap())?;
            }
            self.context_stack.push(context);
            self.has_been_setup_stack.push(true);
            while !self.push_context_stack.is_empty() {
                self.push_context_stack.iter().for_each(|_| self.has_been_setup_stack.push(false));
                self.context_stack.append(&mut self.push_context_stack);
                let mut context = self.context_stack.pop().unwrap();
                self.has_been_setup_stack.pop().unwrap();
                let returned = context.setup(self)?;
                if !returned {
                    self.context_stack.push(context);
                    self.has_been_setup_stack.push(true);
                }
            }
        }
        Ok(())
    }

    /// Set the quit flag.
    pub fn quit(&mut self) {
        self.quit = true;
    }

    /// Push a [Context](super::context::Context) to the stack of contexts.
    pub fn push_context<C: 'a + Context>(&mut self, context: C) {
        self.push_context_stack.push(Box::new(context)); 
    }

    /// Draw the command line.
    ///
    /// (This refers to the last line in the terminal that you type ed commands or ":q" into in
    /// vim. I have no idea what it's really called, so I refer to it as the command line).
    /// 
    /// See also: [`Self::q_draw_cmd_line()`].
    pub fn draw_cmd_line<const N: usize>(&mut self, text: [&str; N]) -> Result<()> {
        self.q_draw_cmd_line(text, true)
    }

    /// Queue the necessary
    /// [`Command`](https://docs.rs/crossterm/latest/crossterm/trait.Command.html)s to draw the command line.
    /// 
    /// See also: [`Self::draw_cmd_line()`].
    pub fn q_draw_cmd_line<const N: usize>(&mut self, text: [&str; N], flush: bool) -> Result<()> {
        let height = self.terminal.size().height;
        self.terminal.cursor_to(0, height - 1).q_move_cursor()?.q(Clear(ClearType::CurrentLine))?;
        for text_bit in text {
            self.terminal.q(Print(text_bit))?;
        }
        if flush { self.terminal.flush() } else { Ok(()) }
    }

    /// Return a reference to the terminal.
    pub fn terminal(&mut self) -> &mut Terminal {
        &mut self.terminal
    }

    /// Push a command to the command history stack.
    pub fn push_command(&mut self, cmd: String) {
        self.command_stack.push(cmd);
    }

    /// Return a reference to the command history stack.
    pub fn command_stack(&self) -> &Vec<String> {
        &self.command_stack
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
        let message_len = WELCOME_SIZE as u16;
        let mut message_line: u16 = 0;
        self.terminal.q(cursor::SavePosition)?.q(cursor::Hide)?.q(Clear(ClearType::All))?;
        for i in 0..(height - 1) {
            if message_line < message_len && i == height / 2 - message_len / 2 + message_line {
                self.terminal.centered_styles("~", &WELCOME_MSG[message_line as usize], "",
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

#[doc(hidden)]
impl<'a> Drop for Editor<'a> {
    fn drop(&mut self) {
        self.terminal.leave_alternate_screen().expect("Failed to leave alternate screen");
    }
}
