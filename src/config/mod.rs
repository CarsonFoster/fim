//! A module for handling configuration files.
//!
//! # Config Format
//!
//! ## Option Lines
//! An option line sets the value of an option (one of the fields of an [`Options`](self::options::Options)
//! object). The syntax of an option line is as follows: `set`, a mandatory space, the name of the option,
//! optional whitespace, `=`, optional whitespace, and the value of the option. In short, an option
//! line is composed like this: `set <OPTION_NAME> = <OPTION_VALUE>`.
//!
//! The name of the option is the name of the corresponding field in the [`Options`](self::options::Options)
//! object. For example, since there is a `line_numbering` field in `Options`, `line_numbering` is the name of
//! the option.
//!
//! The value of the option depends on the type of option: string, numeric, or boolean. You can
//! find the type of an option by looking at the documentation of the fields of
//! [`Options`](self::options::Options) or by looking at the documentation for the associated
//! types of options (e.g. [`LineNumbers`](self::options::LineNumbers). A
//! boolean option's value is either `true` or `false`. A numeric option's value is a 32-bit,
//! signed, decimal integer. A string option's value is a string (but note that leading and trailing
//! whitespace is trimmed from the value). Enums that are declared string options have a
//! restriction on their value: the string value can only be one of their variants (e.g.
//! `Relative`).
//! 
//! ### Examples
//! - `set line_numbering = Relative\r\n`: sets the `line_numbering` field in an
//! [`Options`](self::options::Options) object
//! to be [`LineNumbers::Relative`](self::options::LineNumbers::Relative).
//! - `set layout=Dvorak`: sets the `layout` field in an [`Options`](self::options::Options) object to be
//! [`LayoutType::Dvorak`](self::options::LayoutType::Dvorak).
//!
//! ## `bind` Lines
//! A `bind` line consists of four parts: the bind term, the key event term, the new
//! context term, and the rest of the line (which represents optional arguments).
//! - the `bind` term is formed like this: `bind(<context>)`, where `<context>` represents the name
//! of the context to which this binding applies. For example, if you want to bind a key to perform
//! an action in normal mode, the bind term would be `bind(NormalMode)`.
//!     - this type of `bind` term creates a layout-agnostic key bind. That is, if the current
//!     layout is QWERTY, and you bind `S` to start `CommandMode`, no matter what keyboard layout
//!     fim is currently in, you can press the key location where `S` is in QWERTY (e.g. `O` in Dvorak) to activate
//!     `CommandMode`. This feature allows one to type in a different layout, while retaining
//!     fim/vim QWERTY muscle memory.
//!     - there is also a layout-specific bind term: `bind-layout(<context>)`. This only binds the
//!     key in the current layout. For example: the line `set layout = Dvorak` followed by
//!     `bind-layout(NormalMode) O CommandMode` would bind an `O` key press to start the
//!     CommandMode context only when the current layout is Dvorak.
//! - the key event term represents the key press that you are binding. See below.
//! - the new context term is the name of the context that you want to change to. For example, if
//! you wanted to enter command mode, the new context term would be `CommandMode`.
//! - the optional arguments: no required form overall, specific to each context.
//!
//! ### Key Event Format
//! A key event is either a literal key character (e.g. `A`, `6`, or `/`), one of the following
//! representations of special characters, or a modifier string.
//!
//! ### Special Characters
//! - `<Tab>`: the tab key
//! - `<CR>` or `<Enter>`: the enter key
//! - `<F1>` ... `<F12>`: a function key
//! - `<Ins>`: the insert key
//! - `<Del>`: the delete key (not the backspace key)
//! - `<Home>`: the home key
//! - `<End>`: the end key
//! - `<PageUp>`: the page up key (may read 'PgUp')
//! - `<PageDown>`: the page down key (may read 'PgDn')
//! - `<Left>`, `<Right>`, `<Up>`, `<Down>`: the arrow keys
//! - `<Space>`: a space character / pressing the spacebar
//! - `<BS>`: the backspace key
//! - `<Esc>`: the escape key
//!
//! ### Modifier Strings
//! A modifier string consists of an opening angled bracket, the modifiers (i.e. control, alt,
//! shift), the key press, and a closing angled bracket. The inner key press can be a literal key
//! character or a special character (without the surrounding angled brackets).
//!
//! #### Allowed Modifiers
//! - `C-`: Control is pressed
//! - `A-`: Alt is pressed
//! - `S-`: Shift is pressed
//! - `C-A-`: Control and Alt are pressed
//! - `C-S-`: Control and Shift are pressed
//! - `S-A-`: Shift and Alt are pressed
//! - `C-S-A-`: Control, Shift, and Alt are all pressed
//!
//! #### Examples
//! - `<C-S>`: Control + S
//! - `<C-C>`: Control + C
//! - `<A-Tab>`: Alt + Tab
//! - `<S-A-Enter>`: Shift + Alt + Enter
//! - `<C-S-A-Left>`: Control + Shift + Alt + left arrow key
//! - `<C-A-Del>`: Control + Alt + Delete (this will probably be intercepted by your OS)
//!
//! ## Comments
//! Line comments begin with a `"`. Note that currently all comments must be on their own line.
//!
//! ### Examples
//! - `" this is a comment`
//! - `set line_numbering = Dvorak " I really like Dvorak`: DOES NOT WORK! Place the comment above
//! the option statement.

pub mod config_error;
pub mod keybinds;
pub mod options;

use crate::context::Factory;
use crate::layout::{ Colemak, CustomLayout, Dvorak, Layout, Qwerty };
use self::config_error::{ ConfigParseError, IncludeParseError };
use self::keybinds::KeyBinds;
use self::options::{ LayoutType, Options };
use crossterm::event::KeyEvent;
use std::collections::HashMap;
use std::path::PathBuf;
use std::fs::read_to_string;

/// Struct to hold configuration details of fim.
pub struct Config {
    /// `Options` object.
    pub opt: Options,
    /// `KeyBinds` object.
    pub key_binds: KeyBinds,
    /// Map between layout name and the `CustomLayout` object.
    pub layouts: HashMap<String, CustomLayout>
}

impl Config {
    /// Parse the given config file.
    pub fn new(file: PathBuf) -> Result<Self, ConfigParseError> {
        let mut opt = Options::default();
        let mut key_binds = KeyBinds::new();
        let mut layouts = HashMap::new();
        let string = read_to_string(file)?;
        for (line, line_no) in string.lines().zip(1usize..) {
            Self::parse_line(line, line_no, &mut opt, &mut key_binds, &mut layouts)?;
        }
        Ok(Config{ opt, key_binds, layouts })
    }

    /// Shortcut to calling `self.key_binds.query` with the appropriate arguments.
    pub fn query_binds(&self, context: &str, key: KeyEvent) -> Option<&Factory> {
        self.key_binds.query(context, key, self.opt.layout.clone(), &self.layouts)
    }

    /// Return the trait object reference of the current layout, as according to `opt`.
    pub fn current_layout(&self) -> &dyn Layout {
        Self::layout(&self.opt.layout, &self.layouts)
    }

    /// Convert a `LayoutType` into a `Layout` trait object reference.
    ///
    /// # Panics
    /// Panics if `layout` is a `LayoutType::Custom` and its `name` field is not a key in `map`.
    pub fn layout<'a, 'b>(layout: &'a LayoutType, map: &'b HashMap<String, CustomLayout>) -> &'b dyn Layout {
        match layout {
            LayoutType::Dvorak => &Dvorak as &dyn Layout,
            LayoutType::Colemak => &Colemak as &dyn Layout,
            LayoutType::Qwerty => &Qwerty as &dyn Layout,
            LayoutType::Custom{ name } => map.get(name.as_str()).unwrap() as &dyn Layout
        }
    }

    /// Translate a QWERTY `KeyEvent` into a `KeyEvent` from the current layout.
    pub fn to_current_layout_event(&self, e: KeyEvent) -> KeyEvent {
        let key_code = self.current_layout().from_qwerty_keycode(e.code);
        KeyEvent::new(key_code, e.modifiers)
    }

    /// Convert a `KeyEvent` from a given layout into a QWERTY `KeyEvent`.
    ///
    /// # Panics
    /// Panics if `layout` is a `LayoutType::Custom` and its `name` field is not a key in `map`.
    pub fn to_qwerty_event(e: KeyEvent, layout: &LayoutType, map: &HashMap<String, CustomLayout>) -> KeyEvent {
        let key_code = Self::layout(layout, map).to_qwerty_keycode(e.code);
        KeyEvent::new(key_code, e.modifiers)
    }

    fn parse_line(line: &str, line_no: usize, opt: &mut Options, key_binds: &mut KeyBinds, layouts: &mut HashMap<String, CustomLayout>) -> Result<(), ConfigParseError> {
        // TODO: end of line comments
        if line.trim().len() == 0 || line.starts_with('"') { return Ok(()); }
        if line.starts_with("bind") {
            let result = key_binds.add(line, opt.layout.clone(), layouts);
            if result.is_err() {
                return Err(ConfigParseError::bind(result.unwrap_err(), line_no));
            }
        } else if line.starts_with("set") {
            let result = opt.set_option(line);
            if result.is_err() {
                return Err(ConfigParseError::option(result.unwrap_err(), line_no));
            }
            if let LayoutType::Custom{ name } = &opt.layout {
                if !layouts.contains_key(name.as_str()) {
                    return Err(ConfigParseError::NoMatchingLayout{ line: line_no });
                }
            }
        } else if line.starts_with("include") {
            Self::parse_include(line, line_no, opt, key_binds, layouts)?;
        } else {
            return Err(ConfigParseError::NotAStatement{ line: line_no });
        }
        Ok(())
    }

    fn parse_include(line: &str, line_no: usize, opt: &mut Options, key_binds: &mut KeyBinds, layouts: &mut HashMap<String, CustomLayout>) -> Result<(), ConfigParseError> {
        if let Some(line) = line.strip_prefix("include ") {
            if line.starts_with('\'') {
                // TODO: support regular includes
                Ok(())
            } else if let Some(line) = line.strip_prefix("layout ") {
                if let Some((filename, rest)) = line.strip_prefix('\'').map(|l| l.split_once('\'')).flatten() {
                    let result = CustomLayout::new(filename.parse::<PathBuf>().unwrap());
                    if let Ok(layout) = result {
                        let name = if rest.len() != 0 {
                            if let Some(name) = rest.strip_prefix(" as ").map(|l| l.trim()) {
                                name
                            } else { return Err(ConfigParseError::include(IncludeParseError::MalformedAsClause, line_no)); }
                        } else { layout.name() };
                        layouts.insert(name.to_string(), layout);
                        Ok(())
                    } else {
                        Err(ConfigParseError::layout(result.unwrap_err(), line_no)) 
                    }
                } else { Err(ConfigParseError::include(IncludeParseError::LayoutNoQuotedFile, line_no)) }
            } else { Err(ConfigParseError::include(IncludeParseError::UnknownIncludeType, line_no)) }
        } else { Err(ConfigParseError::include(IncludeParseError::MalformedInclude, line_no)) }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config{ opt: Options::default(), key_binds: KeyBinds::new(), layouts: HashMap::new() }
    }
}
