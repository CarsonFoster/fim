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
    rev_cmd_idx: Option<usize>,
}

impl CommandMode {
    pub fn new() -> CommandMode {
        CommandMode{ str: String::new(), begin: 0, cursor_pos: 0, rev_cmd_idx: None }
    }

    fn terminal_x(&self) -> u16 {
        (1 + self.cursor_pos - self.begin) as u16
    }

    fn begin_for_end(&self, width: u16) -> usize {
        let width: usize = width.into();
        if width > self.str.len() + 1 {
            0
        } else {
            self.str.len() + 1 - width
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
        ed.q_draw_cmd_line([":", &self.str[self.begin..end]], false)
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
            KeyCode::Up => {
                // TODO: memorize current command before first up press
                self.rev_cmd_idx = Some(self.rev_cmd_idx.map_or(0, |i| i + 1));
                let command_stack = ed.command_stack();
                let cmd = self.get_command(command_stack);
                if let Some(cmd) = cmd {
                    self.cursor_pos = cmd.len();
                    self.str = String::from(cmd);
                    self.begin = self.begin_for_end(size.width);
                    self.q_draw(ed)?;
                    self.q_move(ed)?;
                    ed.terminal().flush();
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
                        ed.terminal().flush();
                    } else {
                        self.rev_cmd_idx = self.rev_cmd_idx.map(|i| i + 1);
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
                // TODO: insert at cursor
                if self.cursor_pos >= self.str.len() {
                    self.str.push(character);
                } else {
                    self.str.insert(self.cursor_pos, character);
                }
                self.cursor_pos += 1;
                // one extra character for colon and one extra for cursor
                // TODO: don't always need to leave extra space for cursor
                if self.begin != 0 || self.str.len() + 2 > size.width.into() {
                    self.begin += 1;
                }
                self.q_draw(ed)?;
                self.q_move(ed)?;
                ed.terminal().flush()?;
            },
            otherwise => (),
        }
        Ok(None)
    }
}
