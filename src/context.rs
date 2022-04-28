use crate::editor::Editor;
use crate::terminal::Position;
//use std::io::{Error, ErrorKind};
use std::cmp::min;
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

    fn q_move(&self, ed: &mut Editor) -> Result<()> {
        let height = ed.terminal().size().height;
        ed.terminal().cursor_to(self.terminal_x(), height - 1).q_move_cursor()?;
        Ok(())
    }

    fn q_draw(&self, ed: &mut Editor) -> Result<()> {
        let width: usize = ed.terminal().size().width.into();
        let end = min(self.begin + width - 1, self.str.len());
        ed.q_draw_cmd_line([":", &self.str[self.begin..end]], false)
    }

    fn draw(&self, ed: &mut Editor) -> Result<()> {
        self.q_draw(ed)?;
        ed.terminal().flush()
    }

    fn delete(&mut self, ed: &mut Editor) -> Result<()> {
        self.str.remove(self.cursor_pos.into()); 
        // TODO: adjust begin after delete?
        self.q_draw(ed)?;
        self.q_move(ed)?;
        ed.terminal().flush()
    }
}

impl Context for CommandMode {
    fn setup(&mut self, ed: &mut Editor) -> Result<()> {
        ed.draw_cmd_line([":"])
    }

    fn forward(&mut self, ed: &mut Editor, event: KeyEvent) -> Result<Option<ContextMessage>> {
        let KeyEvent{ code: c, modifiers: m } = event;
        let size = ed.terminal().size();
        match c {
            KeyCode::Enter => {
                // TODO: implement actual logic
                match self.str.as_str() {
                    "q" => ed.quit(),
                    otherwise => (),
                }
                ed.push_command(String::from(&self.str));
                return Ok(Some(ContextMessage::Unit))
            },
            // TODO: history; cursor goes to end
            KeyCode::Up => {

            },
            KeyCode::Down => {

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
                // TODO: insert at cursor
                self.str.push(character);
                self.cursor_pos += 1;
                // one extra character for colon and one extra for cursor
                // TODO: don't always need to leave extra space for cursor
                if self.begin != 0 || self.str.len() + 2 > size.width.into() {
                    self.begin += 1;
                }
                self.draw(ed)?;
            },
            otherwise => (),
        }
        Ok(None)
    }
}
