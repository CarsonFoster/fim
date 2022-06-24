//! A module that contains the main editor logic.
use crate::config::Config;
use crate::config::keybinds::KeyBinds;
use crate::config::options::Options;
use crate::context::*;
use crate::terminal::{Position, Terminal};
use crate::window::Window;
use bitflags::bitflags;
use crossterm::{
    Result,
    terminal::{
        Clear,
        ClearType,
    },
    style::Print,
};
use std::path::PathBuf;

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
    #[doc(hidden)]
    current_window: usize,
    #[doc(hidden)]
    config: Config,
}

impl<'a> Editor<'a> {
    /// Create a new Editor struct from a file.
    pub fn new(filename: PathBuf, config: Option<Config>) -> Result<Editor<'a>> {
        let config = config.unwrap_or_else(|| Config::default());
        let term = Terminal::new()?;
        let window = Window::new(filename, &term, config.opt.clone())?;
        // TODO: add real default config handling
        Ok(Editor{ terminal: term, quit: false, context_stack: vec![Box::new(NormalMode)], push_context_stack: Vec::new(), has_been_setup_stack: vec![true], command_stack: Vec::new(), windows: vec![window], current_window: 0, config })
    }

    /// Create a new Editor struct with the default welcome screen.
    pub fn default(config: Option<Config>) -> Result<Editor<'a>> {
        let config = config.unwrap_or_else(|| Config::default());
        let term = Terminal::new()?;
        let window = Window::default(&term, config.opt.clone());
        Ok(Editor{ terminal: term, quit: false, context_stack: vec![Box::new(NormalMode)], push_context_stack: Vec::new(), has_been_setup_stack: vec![true], command_stack: Vec::new(), windows: vec![window], current_window: 0, config })
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
        self.windows.iter().try_for_each(|w| w.render(&mut self.terminal))?;
        let Position{ x, y } = self.windows[self.current_window].to_term(0, 0);
        self.terminal.move_cursor_to(x, y)
    }

    fn process_keypress(&mut self) -> Result<()> {
        let event = self.config.to_current_layout_event(self.terminal.read_key()?);

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

    /// Actual implementor of [`Action`].
    ///
    /// Necessary due to borrow checker's interaction with disjoint struct fields accessed through
    /// methods.
    pub fn action(&mut self, action: &str) -> Result<()> {
        let current_window = &mut self.windows[self.current_window];
        let term = &mut self.terminal;
        match action {
            "move_left" => current_window.move_left(term)?,
            "move_right" => current_window.move_right(term)?,
            "move_up" => current_window.move_up(term)?,
            "move_down" => current_window.move_down(term)?,
            "home" => current_window.home(term)?,
            "end" => current_window.end(term)?,
            "delete" => current_window.delete(term)?,
            "backspace" => current_window.backspace(term)?,
            _ => (),
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

    /// Push a boxed `Context` to the stack of contexts.
    pub fn push_boxed_context(&mut self, context: Box<dyn Context>) {
        self.push_context_stack.push(context);
    }

    /// Queue the necessary
    /// [`Command`](https://docs.rs/crossterm/latest/crossterm/trait.Command.html)s to draw the command line.
    /// 
    /// Flushes the `Command` only if `flush` is `true`. If `save_cursor` is `true`, saves the
    /// cursor before the draw. If `reset_cursor` is `true`, after the draw
    /// the cursor will be placed where it was previously. Otherwise, it will remain on the command
    /// line after the drawn text.
    /// See also: [`Self::draw_cmd_line()`].
    pub fn q_draw_cmd_line<const N: usize>(&mut self, text: [&str; N], flags: CmdLineFlags) -> Result<()> {
        if flags.contains(CmdLineFlags::SAVECURSOR) { self.terminal.save_cursor(); }
        let height = self.terminal.size().height;
        self.terminal.cursor_to(0, height - 1).q_move_cursor()?.q(Clear(ClearType::CurrentLine))?;
        for text_bit in text {
            self.terminal.q(Print(text_bit))?;
        }
        if flags.contains(CmdLineFlags::RESTORECURSOR) {
            self.terminal.restore_cursor();
            self.terminal.q_move_cursor()?;
        }
        if flags.contains(CmdLineFlags::FLUSH) { self.terminal.flush() } else { Ok(()) }
    }

    /// Return a reference to the terminal.
    pub fn terminal(&mut self) -> &mut Terminal {
        &mut self.terminal
    }

    /// Return a reference to the `KeyBinds` object.
    pub fn key_binds(&self) -> &KeyBinds {
        &self.config.key_binds
    }

    /// Return a reference to the `Options` object.
    pub fn options(&self) -> &Options {
        &self.config.opt
    }

    /// Return a reference to the `Config` object.
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// Push a command to the command history stack.
    pub fn push_command(&mut self, cmd: String) {
        self.command_stack.push(cmd);
    }

    /// Return a reference to the command history stack.
    pub fn command_stack(&self) -> &Vec<String> {
        &self.command_stack
    }

    /// Execute a closure taking mutable `Window` and `Terminal` references on the current window.
    pub fn on_current_window<F, R>(&mut self, f: F) -> Result<R> 
    where
        F: FnOnce(&mut Window, &mut Terminal) -> Result<R> 
    {
        f(&mut self.windows[self.current_window], &mut self.terminal) 
    }
}

#[doc(hidden)]
impl<'a> Drop for Editor<'a> {
    fn drop(&mut self) {
        self.terminal.leave_alternate_screen().expect("Failed to leave alternate screen");
    }
}

bitflags! {
    pub struct CmdLineFlags: u8 {
        const FLUSH         = 0b001;
        const SAVECURSOR    = 0b010;
        const RESTORECURSOR = 0b100;
    }
}
