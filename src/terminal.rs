use std::io::{Stdout, Write, stdout};
use crossterm::{
    Command,
    Result,
    QueueableCommand,
    cursor::{
        MoveTo,
    },
    event::{
        Event,
        KeyEvent,
        read,
    },
    execute,
    style::{
        ContentStyle,
        Print,
    },
    terminal::{
        self,
        EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};

#[derive(Copy, Clone)]
pub struct Size {
    pub width: u16,
    pub height: u16,
}

#[derive(Copy, Clone)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}

pub struct Centered<'a> {
    stdout: &'a mut Stdout,
    term_length: usize,
    prefix: &'a str,
    text: &'a str,
    suffix: &'a str,
    prefix_styles: Option<ContentStyle>,
    text_styles: Option<ContentStyle>,
    suffix_styles: Option<ContentStyle>,
}

impl<'a> Centered<'a> {
    pub fn new(stdout: &'a mut Stdout, term_length: usize, prefix: &'a str, text: &'a str, suffix: &'a str,
               prefix_styles: Option<ContentStyle>, text_styles: Option<ContentStyle>, suffix_styles: Option<ContentStyle>) -> Self {
        Centered{ stdout, term_length, prefix, text, suffix, prefix_styles, text_styles, suffix_styles }
    }

    pub fn q(&mut self) -> Result<()> {
        let length = self.text.len();
        let prefix_length = self.prefix.len();
        let suffix_length = self.suffix.len();
        let no_style = ContentStyle::new();

        if length + prefix_length + suffix_length < self.term_length {
            // everything fits, add padding; extra padding goes on the end
            let padding = self.term_length - length - prefix_length - suffix_length;
            let left_padding = self.term_length / 2 - length / 2 - prefix_length;
            let right_padding = padding - left_padding;
            self.stdout.queue(Print(self.prefix_styles.map_or_else(|| no_style.apply(self.prefix), |s| s.apply(self.prefix))))?;
            self.stdout.queue(Print(" ".repeat(left_padding)))?;
            self.stdout.queue(Print(self.text_styles.map_or_else(|| no_style.apply(self.text), |s| s.apply(self.text))))?;
            self.stdout.queue(Print(" ".repeat(right_padding)))?;
            self.stdout.queue(Print(self.suffix_styles.map_or_else(|| no_style.apply(self.suffix), |s| s.apply(self.suffix))))?;
        } else if length < self.term_length {
            // at least the main self.text fits
            // adds as many self.prefix and self.suffix characters as possible; if self.prefix/self.suffix runs out,
            // padding is used for the remaining characters instead
            // extra character goes on left
            let remainder = self.term_length - length;
            let right_length = remainder / 2;
            let left_length = remainder - right_length;
            if self.prefix.len() <= left_length {
                self.stdout.queue(Print(self.prefix_styles.map_or_else(|| no_style.apply(self.prefix), |s| s.apply(self.prefix))))?;
                self.stdout.queue(Print(" ".repeat(left_length - self.prefix.len())))?;
            } else {
                let string = &self.prefix[0..left_length];
                self.stdout.queue(Print(self.prefix_styles.map_or_else(|| no_style.apply(string), |s| s.apply(string))))?;
            }
            self.stdout.queue(Print(self.text_styles.map_or_else(|| no_style.apply(self.text), |s| s.apply(self.text))))?;
            if self.suffix.len() <= right_length {
                self.stdout.queue(Print(" ".repeat(right_length - self.suffix.len())))?;
                self.stdout.queue(Print(self.suffix_styles.map_or_else(|| no_style.apply(self.suffix), |s| s.apply(self.suffix))))?;
            } else {
                let string = &self.suffix[0..right_length];
                self.stdout.queue(Print(self.suffix_styles.map_or_else(|| no_style.apply(string), |s| s.apply(string))))?;
            }
        } else {
            // main text doesn't fit, include self.term_length characters from the middle
            // bias toward extra character on the beginning
            let removed_chars = length - self.term_length;
            let skip = removed_chars / 2;
            let bytes: Vec<u8> = self.text.bytes().skip(skip).take(self.term_length).collect(); 
            let string = String::from_utf8(bytes).expect("centered expects only ASCII!");
            let slice: &str = string.as_ref();
            self.stdout.queue(Print(self.text_styles.map_or_else(|| no_style.apply(slice), |s| s.apply(slice))))?;
        }
        Ok(())
    }
}

pub struct Terminal {
    size: Size,
    stdout: Stdout,
    cursor_pos: Position,
    cursor_stack: Vec<Position>,
}

impl Terminal {
    pub fn new() -> Result<Self> {
        let (width, height) = terminal::size()?;
        let stdout = stdout();
        terminal::enable_raw_mode()?;
        Ok( Terminal{ stdout, size: Size{ width, height }, cursor_pos: Position{ x: 0, y: 0 }, cursor_stack: Vec::new() } )
    }

    fn saturating_sub(x: u16, d: u16) -> u16 {
        if d > x {
            0
        } else {
            x - d
        }
    }

    fn set_x(&mut self, x: u16) {
        if x < self.size.width {
            self.cursor_pos.x = x;
        }
    }

    fn set_y(&mut self, y: u16) {
        if y < self.size.height {
            self.cursor_pos.y = y;
        }
    }

    pub fn size(&self) -> Size {
        self.size
    }

    pub fn cursor_pos(&self) -> &Position {
        &self.cursor_pos
    }

    pub fn save_cursor(&mut self) {
        self.cursor_stack.push(self.cursor_pos);
    }

    pub fn restore_cursor(&mut self) {
        if let Some(pos) = self.cursor_stack.pop() {
            self.cursor_pos = pos;
        }
    }

    pub fn move_cursor(&mut self) -> Result<()> { 
        execute!(self.stdout, MoveTo(self.cursor_pos.x, self.cursor_pos.y))
    }

    pub fn q_move_cursor(&mut self) -> Result<&mut Self> {
        self.stdout.queue(MoveTo(self.cursor_pos.x, self.cursor_pos.y))?;
        Ok(self)
    }

    pub fn move_cursor_to(&mut self, x: u16, y: u16) -> Result<()> {
        self.cursor_to(x, y);
        self.move_cursor()
    }

    pub fn cursor_to(&mut self, x: u16, y: u16) -> &mut Self {
        self.set_x(x);
        self.set_y(y);
        self
    }

    pub fn cursor_left_by(&mut self, d_x: u16) -> &mut Self {
        self.cursor_to(Self::saturating_sub(self.cursor_pos.x, d_x), self.cursor_pos.y);
        self
    }

    pub fn cursor_right_by(&mut self, d_x: u16) -> &mut Self {
        self.cursor_to(self.cursor_pos.x.saturating_add(d_x), self.cursor_pos.y);
        self
    }

    pub fn cursor_up_by(&mut self, d_y: u16) -> &mut Self {
        self.cursor_to(self.cursor_pos.x, Self::saturating_sub(self.cursor_pos.y, d_y));
        self
    }

    pub fn cursor_down_by(&mut self, d_y: u16) -> &mut Self {
        self.cursor_to(self.cursor_pos.x, self.cursor_pos.y.saturating_add(d_y));
        self
    }
    

    pub fn enter_alternate_screen(&mut self) -> Result<()> {
        execute!(self.stdout, EnterAlternateScreen)
    }

    pub fn leave_alternate_screen(&mut self) -> Result<()> {
        execute!(self.stdout, LeaveAlternateScreen)
    }

    pub fn q(&mut self, cmd: impl Command) -> Result<&mut Self> {
        self.stdout.queue(cmd)?;
        Ok(self)
    }

    pub fn flush(&mut self) -> Result<()> {
        self.stdout.flush()
    }

    pub fn read_key(&self) -> Result<KeyEvent> {
        loop {
            let event = read()?;
            if let Event::Key(key_event) = event {
                return Ok(key_event);
            }
        }
    }

    pub fn centered_styles<'a>(&'a mut self, prefix: &'a str, text: &'a str, suffix: &'a str,
                      prefix_styles: Option<ContentStyle>, text_styles: Option<ContentStyle>, suffix_styles: Option<ContentStyle>) -> Centered<'a> {
        Centered::new(&mut self.stdout, self.size.width as usize, prefix, text, suffix, prefix_styles, text_styles, suffix_styles)  
    }

    pub fn centered(&self, prefix: &str, text: &str, suffix: &str) -> String {
        // NOTE: does not deal with graphemes/unicode! ASCII only
        // nothing should overflow; a usize amount of text is an insane amount
        let length = text.len();
        let term_length = self.size.width as usize;
        let prefix_length = prefix.len();
        let suffix_length = suffix.len();

        let mut result = String::with_capacity(term_length);
        if length + prefix_length + suffix_length < term_length {
            // everything fits, add padding; extra padding goes on the end
            let padding = term_length - length - prefix_length - suffix_length;
            let left_padding = term_length / 2 - length / 2 - prefix_length;
            let right_padding = padding - left_padding;
            result.push_str(prefix);
            for _ in 0..left_padding {
                result.push_str(" ");
            }
            result.push_str(text);
            for _ in 0..right_padding {
                result.push_str(" ");
            }
            result.push_str(suffix);
        } else if length < term_length {
            // at least the main text fits
            // adds as many prefix and suffix characters as possible; if prefix/suffix runs out,
            // padding is used for the remaining characters instead
            // extra character goes on left
            let remainder = term_length - length;
            let right_length = remainder / 2;
            let left_length = remainder - right_length;
            if prefix.len() <= left_length {
                result.push_str(prefix);
                for _ in 0..(left_length - prefix.len()) {
                    result.push_str(" ");
                }
            } else {
                result.push_str(&prefix[0..left_length]);
            }
            result.push_str(text);
            if suffix.len() <= right_length {
                for _ in 0..(right_length - suffix.len()) {
                    result.push_str(" ");
                }
                result.push_str(suffix);
            } else {
                result.push_str(&suffix[0..right_length]);
            }
        } else {
            // main text doesn't fit, include term_length characters from the middle
            // bias toward extra character on the beginning
            let removed_chars = length - term_length;
            let skip = removed_chars / 2;
            let bytes: Vec<u8> = text.bytes().skip(skip).take(term_length).collect(); 
            result = String::from_utf8(bytes).expect("centered expects only ASCII!");
        }
        result
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Failed to disable raw mode.");
    }
}
