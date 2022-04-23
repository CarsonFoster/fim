use crate::editor::Editor;
use crossterm::{
    Result,
    event::{KeyCode, KeyEvent, KeyModifiers},
};

pub enum ContextMessage {
    Str(String),
    BitSet(u32),
    Int(i32),
    Float(f32),
    Bool(bool),
}

pub trait Context {
    fn forward(&mut self, ed: &mut Editor) -> Result<Option<ContextMessage>>;
    fn receive(&mut self, ed: &mut Editor, arg: ContextMessage) -> Result<Option<ContextMessage>>;
}
