use crate::editor::Editor;
use crossterm::{
    Result,
    event::{KeyCode, KeyEvent, KeyModifiers},
};

pub enum ContextMessage {
    Unit,
    Str(String),
    BitSet(u32),
    Int(i32),
    Float(f32),
    Bool(bool),
}

pub trait Context {
    fn forward(&mut self, ed: &mut Editor, event: KeyEvent) -> Result<Option<ContextMessage>> {
        Ok(None)
    }

    fn receive(&mut self, ed: &mut Editor, arg: ContextMessage) -> Result<Option<ContextMessage>> {
        Ok(None)
    }
}

pub struct NormalMode;
impl Context for NormalMode {
    fn forward(&mut self, ed: &mut Editor, event: KeyEvent) -> Result<Option<ContextMessage>> {
        let KeyEvent{ code: c, modifiers: m } = event;
        if c == KeyCode::Char(':') {
            ed.push_context(CommandMode::new()); 
        }
        Ok(None)
    }
}

pub struct CommandMode {
    str: String,
}

impl CommandMode {
    pub fn new() -> CommandMode {
        CommandMode{ str: String::new() }
    }
}

impl Context for CommandMode {
    fn forward(&mut self, ed: &mut Editor, event: KeyEvent) -> Result<Option<ContextMessage>> {
        let KeyEvent{ code: c, modifiers: m } = event;
        match c {
            KeyCode::Enter => {
                // TODO: implement actual logic
                match self.str.as_str() {
                    "q" => ed.quit(),
                    otherwise => (),
                }
                return Ok(Some(ContextMessage::Unit))
            },
            KeyCode::Backspace => {

            },
            KeyCode::Delete => {

            },
            KeyCode::Char(character) => {
                self.str.push(character);
                // TODO: display command line
            },
            otherwise => (), // TODO: implement arrow keys and cursor
        }
        Ok(None)
    }
}
