//! A module that contains the logic for 'Contexts'.
//!
//! [`Context`]s essentially represent fim's different modes and commands. Only one [`Context`] is active
//! at a time on the 'context stack'. [`Context`]s can interact with the [`Context`]s one above and
//! one below them on the context stack, once. See the [`Context`] page for more details on the
//! interaction between [`Context`]s. 
//!
//! [`Context`]s are 'forwarded' all key presses while they are the active [`Context`]. They are
//! free to do as they choose with the key presses, and can even push new active [`Context`]s. When
//! a [`Context`] 'returns', that return message is 'received' by the [`Context`] immediately
//! beneath it on the context stack (the [`Context`] that becomes the active context after this one
//! is popped). [`Context`]s also have a 'setup' function that is called once, at the time that
//! [`Context`] becomes the active [`Context`].
use crate::editor::{CmdLineFlags, Editor};
use std::cmp::min;
use crossterm::{
    Result,
    event::{KeyCode, KeyEvent},
};

/// Enum for return values of [`Context`]s.
pub enum ContextMessage {
    /// Indicates no return value, analogous to 'void' in C-like languages.
    Unit,
    /// A String return value.
    Str(String),
    /// A 32-bit unsigned integer return value, intended to be used as a bit set.
    BitSet(u32),
    /// A 32-bit signed integer return value.
    Int(i32),
    /// A 32-bit signed floating point return value.
    Float(f32),
    /// A boolean return value.
    Bool(bool),
}

/// Trait to represent contexts.
///
/// See [module-level documentation](index.html) for more information.
pub trait Context {
    /// Function to setup the Context when it becomes the active context.
    ///
    /// Can cause the Context to 'return' and be popped off the context stack, if this function
    /// returns Ok(true). If this function returns Ok(false), the Context has not 'returned'.
    fn setup(&mut self, _ed: &mut Editor) -> Result<bool> {
        Ok(false)
    }

    /// Accepts forwarded key presses.
    ///
    /// Can cause the Context to 'return', if this function returns `Ok(Some(c))`, where `c` is a
    /// [`ContextMessage`].
    fn forward(&mut self, _ed: &mut Editor, _event: KeyEvent) -> Result<Option<ContextMessage>> {
        Ok(None)
    }

    /// Receives the return value of the [`Context`] above it on the context stack.
    ///
    /// Can cause the Context to 'return', if this function returns `Ok(Some(c))` where `c` is a
    /// [`ContextMessage`].
    fn receive(&mut self, _ed: &mut Editor, _arg: ContextMessage) -> Result<Option<ContextMessage>> {
        Ok(None)
    }
}

/// Wrapper type for functions that create [`Context`]s.
pub struct Factory {
    #[doc(hidden)]
    ptr: Box<dyn Fn() -> Box<dyn Context>>
}

impl Factory {
    /// Create a new [`Factory`] from a function.
    pub fn new<T>(ptr: impl Fn() -> T + 'static) -> Self
    where T: Context + Sized + 'static
    {
        Factory{ ptr: Box::new(move || Box::new(ptr())) }
    }

    /// Create a new [`Factory`] from a function that returns a boxed [`Context`].
    pub fn from(ptr: impl Fn() -> Box<dyn Context> + 'static) -> Self {
        Factory{ ptr: Box::new(ptr) }
    }

    /// Create a new [`Context`] using the wrapped function.
    pub fn create(&self) -> Box<dyn Context> {
        (self.ptr)()
    }
}

/// Maps between Strings and Contexts.
///
/// # Arguments
/// - `name`: the name of the Context
/// - `args`: any additional arguments that need to be passed to the Context
/// # Return
/// Returns `None` if there is no context with the name `name`. Returns a [`Factory`] otherwise.
pub fn context(name: &str, args: String) -> Option<Factory> {
    match name {
        "NormalMode" => Some(Factory::new(|| NormalMode)),
        "CommandMode" => Some(Factory::new(|| CommandMode::new())),
        "Action" => Some(Factory::new(move || Action::new(String::from(&args)))),
        "InsertMode" => Some(Factory::new(|| InsertMode)),
        _ => None
    }
}

/// Struct that represents a "one-and-done" action context.
///
/// This is the context that does one action in its `setup` method, and then returns. This is not
/// for state-like contexts (e.g. `NormalMode` or `CommandMode`).
///
///
pub struct Action {
    #[doc(hidden)]
    action: String
}

impl Action {
    /// Create a new `Action` corresponding to the passed string.
    pub fn new(action: String) -> Self {
        Action{ action }
    }
}

impl Context for Action {
    fn setup(&mut self, ed: &mut Editor) -> Result<bool> {
        ed.action(self.action.as_str())?;
        Ok(true)
    }
}

/// Struct that represents fim's NormalMode context.
///
/// Analogous to vim's normal mode. This context always starts as the active context, and there is
/// always one instance of this struct at the bottom of the context stack.
pub struct NormalMode;
impl Context for NormalMode {
    fn forward(&mut self, ed: &mut Editor, event: KeyEvent) -> Result<Option<ContextMessage>> {
        if let Some(factory) = ed.config().query_binds("NormalMode", event) {
            let context = factory.create();
            ed.push_boxed_context(context);
        }
        Ok(None)
    }
}

/// Struct that represents fim's CommandMode context.
///
/// This is the context that (in vim) would allow you to enter ed commands or ':q'. I am not yet
/// decided whether fim will use ed commands or something different, but this is still the context
/// where you enter commands after a ':' at the bottom of the screen.
pub struct CommandMode {
    #[doc(hidden)]
    str: String,
    #[doc(hidden)]
    begin: usize,
    #[doc(hidden)]
    cursor_pos: usize,
    #[doc(hidden)]
    rev_cmd_idx: Option<usize>,
    #[doc(hidden)]
    saved_str: Option<String>,
}

impl CommandMode {
    /// Create a new CommandMode instance.
    pub fn new() -> CommandMode {
        CommandMode{ str: String::new(), begin: 0, cursor_pos: 0, rev_cmd_idx: None, saved_str: None }
    }

    fn terminal_x(&self) -> u16 {
        (1 + self.cursor_pos - self.begin) as u16
    }

    fn begin_for_end(&self, width: u16) -> usize {
        let width: usize = width.into();
        if width > self.str.len() + 2 {
            0
        } else {
            self.str.len() + 2 - width
        }
    }

    fn q_move(&self, ed: &mut Editor) -> Result<()> {
        let height = ed.terminal().size().height;
        ed.terminal().cursor_to(self.terminal_x(), height - 1).q_move_cursor()?;
        Ok(())
    }

    fn q_draw(&self, ed: &mut Editor) -> Result<()> {
        let width: usize = ed.terminal().size().width.into();
        let end = min(self.begin + width - 1, self.str.len());
        ed.q_draw_cmd_line([":", &self.str[self.begin..end]], CmdLineFlags::empty())
    }

    fn delete(&mut self, ed: &mut Editor) -> Result<()> {
        self.str.remove(self.cursor_pos.into()); 
        // TODO: adjust begin after delete?
        self.q_draw(ed)?;
        self.q_move(ed)?;
        ed.terminal().flush()
    }

    fn get_command<'a>(&self, stack: &'a Vec<String>) -> Option<&'a String> {
        if let Some(idx) = self.rev_cmd_idx {
            if stack.len() < idx + 1 { None } else { stack.get(stack.len() - idx - 1) }
        } else {
            None
        }
    }
}

impl Context for CommandMode {
    fn setup(&mut self, ed: &mut Editor) -> Result<bool> {
        ed.q_draw_cmd_line([":"], CmdLineFlags::FLUSH | CmdLineFlags::SAVECURSOR)?;
        Ok(false)
    }

    fn forward(&mut self, ed: &mut Editor, event: KeyEvent) -> Result<Option<ContextMessage>> {
        let KeyEvent{ code: c, modifiers: _ } = event;
        let size = ed.terminal().size();
        match c {
            KeyCode::Enter => {
                // TODO: implement actual logic
                match self.str.as_str() {
                    "q" => ed.quit(),
                    _ => (),
                }
                ed.q_draw_cmd_line([], CmdLineFlags::FLUSH | CmdLineFlags::RESTORECURSOR)?;
                ed.push_command(String::from(&self.str));
                return Ok(Some(ContextMessage::Unit))
            },
            KeyCode::Up => {
                if self.rev_cmd_idx.is_none() {
                    self.saved_str = Some(String::from(&self.str));
                }
                self.rev_cmd_idx = Some(self.rev_cmd_idx.map_or(0, |i| i + 1));
                let command_stack = ed.command_stack();
                let cmd = self.get_command(command_stack);
                if let Some(cmd) = cmd {
                    self.cursor_pos = cmd.len();
                    self.str = String::from(cmd);
                    self.begin = self.begin_for_end(size.width);
                    self.q_draw(ed)?;
                    self.q_move(ed)?;
                    ed.terminal().flush()?;
                } else {
                    self.rev_cmd_idx = if self.rev_cmd_idx.unwrap() == 0 { None } else { Some(self.rev_cmd_idx.unwrap() - 1) };
                }
            },
            KeyCode::Down => {
                if self.rev_cmd_idx.unwrap_or_default() > 0 {
                    self.rev_cmd_idx = self.rev_cmd_idx.map(|i| i - 1);
                    let cmd = self.get_command(ed.command_stack());
                    if let Some(cmd) = cmd {
                        self.cursor_pos = cmd.len();
                        self.str = String::from(cmd);
                        self.begin = self.begin_for_end(size.width);
                        self.q_draw(ed)?;
                        self.q_move(ed)?;
                        ed.terminal().flush()?;
                    } else {
                        self.rev_cmd_idx = self.rev_cmd_idx.map(|i| i + 1);
                    }
                } else if let Some(0) = self.rev_cmd_idx {
                    if let Some(cmd) = self.saved_str.take() {
                        self.rev_cmd_idx = None;
                        self.cursor_pos = cmd.len();
                        self.str = cmd;
                        self.begin = self.begin_for_end(size.width);
                        self.q_draw(ed)?;
                        self.q_move(ed)?;
                        ed.terminal().flush()?;
                    }
                }
            },
            KeyCode::Left => {
                if self.cursor_pos > 0 {
                    if self.terminal_x() == 1 && self.begin != 0 {
                        self.begin -= 1;
                        self.q_draw(ed)?;
                    } else {
                        self.cursor_pos -= 1;
                        self.q_move(ed)?;
                    }
                    ed.terminal().flush()?;
                }
            },
            KeyCode::Right => {
                if self.cursor_pos < self.str.len() {
                    if self.terminal_x() + 1 == size.width {
                        self.begin += 1;
                        self.q_draw(ed)?;
                    } else {
                        self.cursor_pos += 1;
                        self.q_move(ed)?;
                    }
                    ed.terminal().flush()?;
                }
            },
            KeyCode::Backspace => {
                if self.cursor_pos > 0 {
                    self.cursor_pos -= 1;
                    self.delete(ed)?;
                }
            },
            KeyCode::Delete => {
                if self.cursor_pos < self.str.len() {
                    self.delete(ed)?;
                }
            },
            KeyCode::Char(character) => {
                if (self.begin == 0 && self.cursor_pos + 2 == size.width.into()) || (self.begin != 0 && self.cursor_pos == self.str.len()) {
                    self.begin += 1;
                }
                if self.cursor_pos >= self.str.len() {
                    self.str.push(character);
                } else {
                    self.str.insert(self.cursor_pos, character);
                }
                self.cursor_pos += 1;
                self.q_draw(ed)?;
                self.q_move(ed)?;
                ed.terminal().flush()?;
            },
            _ => (),
        }
        Ok(None)
    }
}

pub struct InsertMode;

impl Context for InsertMode {
    fn setup(&mut self, ed: &mut Editor) -> Result<bool> {
        ed.q_draw_cmd_line(["-- INSERT --"], CmdLineFlags::all())?;
        Ok(false)
    }

    fn forward(&mut self, ed: &mut Editor, key: KeyEvent) -> Result<Option<ContextMessage>> {
        // matches built-in binds first, then checks for user binds, and then checks for chars
        let code = key.code;
        match code {
            KeyCode::Enter => (),
            KeyCode::Tab => (),
            KeyCode::Esc => {
                ed.q_draw_cmd_line([], CmdLineFlags::all())?;
                return Ok(Some(ContextMessage::Unit));
            },
            _ => {
                if let Some(factory) = ed.config().query_binds("InsertMode", key) {
                    let context = factory.create();
                    ed.push_boxed_context(context);
                } else {
                    match code {
                        KeyCode::Char(c) => ed.on_current_window(|w, t| w.insert(c, t)).map(|_| ())?,
                        _ => ()
                    }
                }
            }
        }
        Ok(None)
    }
}
