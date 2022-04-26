use crate::editor::Editor;
use crate::terminal::Position;
//use std::io::{Error, ErrorKind};
use crossterm::{
    Result,
    cursor::{SavePosition, RestorePosition},
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
    fn setup(&mut self, ed: &mut Editor) -> Result<()> {
        Ok(())
    }
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
    begin: usize,
    cursor_pos: usize,
}

impl CommandMode {
    pub fn new() -> CommandMode {
        CommandMode{ str: String::new(), begin: 0, cursor_pos: 0 }
    }

    fn terminal_x(&self) -> u16 {
        (1 + self.cursor_pos - self.begin) as u16
    }

    fn delete(&mut self, ed: &mut Editor) -> Result<()> {
        let size = ed.terminal().size();
        self.str.remove(self.cursor_pos.into()); 
        // TODO: adjust begin after delete?
        ed.q_draw_cmd_line([":", &self.str[self.begin..]], false)?;
        ed.terminal().cursor_to(self.terminal_x(), size.height - 1);
        ed.terminal().q_move_cursor()?;
        ed.terminal().flush()
    }
}

impl Context for CommandMode {
    fn setup(&mut self, ed: &mut Editor) -> Result<()> {
        ed.draw_cmd_line([":"])
    }

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
                if self.cursor_pos > 0 {
                    self.cursor_pos -= 1;
                    self.delete(ed)?;
                }
            },
            KeyCode::Delete => {

            },
            KeyCode::Char(character) => {
                self.str.push(character);
                self.cursor_pos += 1;
                // one extra character for colon and one extra for cursor
                // TODO: don't always need to leave extra space for cursor
                if self.begin != 0 || self.str.len() + 2 > ed.terminal().size().width.into() {
                    self.begin += 1;
                }
                ed.draw_cmd_line([":", &self.str[self.begin..]])?;
            },
            otherwise => (), // TODO: implement arrow keys and cursor
        }
        Ok(None)
    }
}
