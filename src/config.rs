//! A module for handling configuration files that map keyboard presses to contexts.
//!
//! # Config Format
//!
//! ## Option Lines
//! An option line sets the value of an option (one of the fields of an [`Options`](crate::options::Options)
//! object). The syntax of an option line is as follows: `set`, a mandatory space, the name of the option,
//! optional whitespace, `=`, optional whitespace, and the value of the option. In short, an option
//! line is composed like this: `set <OPTION_NAME> = <OPTION_VALUE>`.
//!
//! The name of the option is the name of the corresponding field in the [`Options`](crate::options::Options)
//! object. For example, since there is a `line_numbering` field in `Options`, `line_numbering` is the name of
//! the option.
//!
//! The value of the option depends on the type of option: string, numeric, or boolean. You can
//! find the type of an option by looking at the documentation of the fields of
//! [`Options`](crate::options::Options) or by looking at the documentation for the associated
//! types of options (e.g. [`LineNumbers`](crate::options::LineNumbers). A
//! boolean option's value is either `true` or `false`. A numeric option's value is a 32-bit,
//! signed, decimal integer. A string option's value is a string (but note that leading and trailing
//! whitespace is trimmed from the value). Enums that are declared string options have a
//! restriction on their value: the string value can only be one of their variants (e.g.
//! `Relative`).
//! 
//! ### Examples
//! - `set line_numbering = Relative\r\n`: sets the `line_numbering` field in an
//! [`Options`](crate::options::Options) object
//! to be [`LineNumbers::Relative`](crate::options::LineNumbers::Relative).
//! - `set layout=Dvorak`: sets the `layout` field in an [`Options`](crate::options::Options) object to be
//! [`LayoutType::Dvorak`](crate::options::LayoutType::Dvorak).
//!
//! ## `bind` Lines
//! A `bind` line consists of four parts: the bind term, the key event term, the new
//! context term, and the rest of the line (which represents optional arguments).
//! - the `bind` term is formed like this: `bind(<context>)`, where `<context>` represents the name
//! of the context to which this binding applies. For example, if you want to bind a key to perform
//! an action in normal mode, the bind term would be `bind(NormalMode)`.
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

use crate::config_error::*;
use crate::context::*;
use std::path::PathBuf;
use std::fs::read_to_string;
use std::collections::HashMap;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

struct ConfigMap {
    unprintable: HashMap<String, KeyEvent>,
}

impl ConfigMap {
    fn new() -> Self {
        let mut unprintable = HashMap::new();
        let pairs = [("BS", KeyCode::Backspace), ("CR", KeyCode::Enter), ("Enter", KeyCode::Enter), ("Left", KeyCode::Left),
                     ("Right", KeyCode::Right), ("Up", KeyCode::Up), ("Down", KeyCode::Down), ("Home", KeyCode::Home),
                     ("End", KeyCode::End), ("PageUp", KeyCode::PageUp), ("PageDown", KeyCode::PageDown), ("Tab", KeyCode::Tab),
                     ("Del", KeyCode::Delete), ("Ins", KeyCode::Insert), ("Space", KeyCode::Char(' ')), ("Esc", KeyCode::Esc)];
        for (rep, code) in pairs {
            unprintable.insert(rep.to_string(), KeyEvent::new(code, KeyModifiers::NONE));
        }
        for i in 1u8..=12 {
            unprintable.insert(format!("F{}", i), KeyEvent::new(KeyCode::F(i), KeyModifiers::NONE));
        }
        ConfigMap{ unprintable }
    }

    pub fn query(&self, rep: &str) -> Option<KeyEvent> {
        if rep.len() == 0 {
            return None;
        }
        let head = rep.chars().next();
        let middle = rep.get(1..rep.len() - 1);
        if rep.len() == 1 && head.is_some() && head.unwrap().is_ascii_graphic() {
            Some(KeyEvent::new(KeyCode::Char(head.unwrap().to_ascii_uppercase()), KeyModifiers::NONE))
        } else if rep.len() > 2 && rep.get(0..1) == Some("<") && rep.get(rep.len() - 1..) == Some(">")
            && middle.is_some() && self.unprintable.contains_key(middle.unwrap()) {
            Some(self.unprintable[middle.unwrap()])
        } else {
            None
        }
    }

    pub fn query_code(&self, rep: &str) -> Option<KeyCode> {
        if rep.len() == 0 {
            return None;
        }
        let head = rep.chars().next();
        if rep.len() == 1 && head.is_some() && head.unwrap().is_ascii_graphic() {
            Some(KeyCode::Char(head.unwrap().to_ascii_uppercase()))
        } else if self.unprintable.contains_key(rep) {
            Some(self.unprintable[rep].code)
        } else {
            None
        }
    }
}

lazy_static! {
    static ref MAP: ConfigMap = ConfigMap::new();
}

#[test]
fn test_config_map() {
    let ascii_graphics = "`1234567890-=~!@#$%^&*()_+qwertyuiop[]QWERTYUIOP{}asdfghjkl;'\\ASDFGHJKL:\"|zxcvbnm,./ZXCVBNM<>?";
    for ch in ascii_graphics.chars() {
        assert_eq!(MAP.query(&ch.to_string()), Some(KeyEvent::new(KeyCode::Char(ch.to_ascii_uppercase()), KeyModifiers::NONE)));
        assert_eq!(MAP.query_code(&ch.to_string()), Some(KeyCode::Char(ch.to_ascii_uppercase())));
    }
    assert_eq!(MAP.query("<Tab>"), Some(KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE)));
    assert_eq!(MAP.query_code("Tab"), Some(KeyCode::Tab));
    assert_eq!(MAP.query("<CR>"), Some(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE)));
    assert_eq!(MAP.query_code("CR"), Some(KeyCode::Enter));
    assert_eq!(MAP.query("<F1>"), Some(KeyEvent::new(KeyCode::F(1), KeyModifiers::NONE)));
    assert_eq!(MAP.query_code("F1"), Some(KeyCode::F(1)));
    assert_eq!(MAP.query("<F2>"), Some(KeyEvent::new(KeyCode::F(2), KeyModifiers::NONE)));
    assert_eq!(MAP.query_code("F2"), Some(KeyCode::F(2)));
    assert_eq!(MAP.query("<F3>"), Some(KeyEvent::new(KeyCode::F(3), KeyModifiers::NONE)));
    assert_eq!(MAP.query_code("F3"), Some(KeyCode::F(3)));
    assert_eq!(MAP.query("<F4>"), Some(KeyEvent::new(KeyCode::F(4), KeyModifiers::NONE)));
    assert_eq!(MAP.query_code("F4"), Some(KeyCode::F(4)));
    assert_eq!(MAP.query("<F5>"), Some(KeyEvent::new(KeyCode::F(5), KeyModifiers::NONE)));
    assert_eq!(MAP.query_code("F5"), Some(KeyCode::F(5)));
    assert_eq!(MAP.query("<F6>"), Some(KeyEvent::new(KeyCode::F(6), KeyModifiers::NONE)));
    assert_eq!(MAP.query_code("F6"), Some(KeyCode::F(6)));
    assert_eq!(MAP.query("<F7>"), Some(KeyEvent::new(KeyCode::F(7), KeyModifiers::NONE)));
    assert_eq!(MAP.query_code("F7"), Some(KeyCode::F(7)));
    assert_eq!(MAP.query("<F8>"), Some(KeyEvent::new(KeyCode::F(8), KeyModifiers::NONE)));
    assert_eq!(MAP.query_code("F8"), Some(KeyCode::F(8)));
    assert_eq!(MAP.query("<F9>"), Some(KeyEvent::new(KeyCode::F(9), KeyModifiers::NONE)));
    assert_eq!(MAP.query_code("F9"), Some(KeyCode::F(9)));
    assert_eq!(MAP.query("<F10>"), Some(KeyEvent::new(KeyCode::F(10), KeyModifiers::NONE)));
    assert_eq!(MAP.query_code("F10"), Some(KeyCode::F(10)));
    assert_eq!(MAP.query("<F11>"), Some(KeyEvent::new(KeyCode::F(11), KeyModifiers::NONE)));
    assert_eq!(MAP.query_code("F11"), Some(KeyCode::F(11)));
    assert_eq!(MAP.query("<F12>"), Some(KeyEvent::new(KeyCode::F(12), KeyModifiers::NONE)));
    assert_eq!(MAP.query_code("F12"), Some(KeyCode::F(12)));
    assert_eq!(MAP.query("<Ins>"), Some(KeyEvent::new(KeyCode::Insert, KeyModifiers::NONE)));
    assert_eq!(MAP.query_code("Ins"), Some(KeyCode::Insert));
    assert_eq!(MAP.query("<Del>"), Some(KeyEvent::new(KeyCode::Delete, KeyModifiers::NONE)));
    assert_eq!(MAP.query_code("Del"), Some(KeyCode::Delete));
    assert_eq!(MAP.query("<Home>"), Some(KeyEvent::new(KeyCode::Home, KeyModifiers::NONE)));
    assert_eq!(MAP.query_code("Home"), Some(KeyCode::Home));
    assert_eq!(MAP.query("<End>"), Some(KeyEvent::new(KeyCode::End, KeyModifiers::NONE)));
    assert_eq!(MAP.query_code("End"), Some(KeyCode::End));
    assert_eq!(MAP.query("<PageUp>"), Some(KeyEvent::new(KeyCode::PageUp, KeyModifiers::NONE)));
    assert_eq!(MAP.query_code("PageUp"), Some(KeyCode::PageUp));
    assert_eq!(MAP.query("<PageDown>"), Some(KeyEvent::new(KeyCode::PageDown, KeyModifiers::NONE)));
    assert_eq!(MAP.query_code("PageDown"), Some(KeyCode::PageDown));
    assert_eq!(MAP.query("<Left>"), Some(KeyEvent::new(KeyCode::Left, KeyModifiers::NONE)));
    assert_eq!(MAP.query_code("Left"), Some(KeyCode::Left));
    assert_eq!(MAP.query("<Right>"), Some(KeyEvent::new(KeyCode::Right, KeyModifiers::NONE)));
    assert_eq!(MAP.query_code("Right"), Some(KeyCode::Right));
    assert_eq!(MAP.query("<Up>"), Some(KeyEvent::new(KeyCode::Up, KeyModifiers::NONE)));
    assert_eq!(MAP.query_code("Up"), Some(KeyCode::Up));
    assert_eq!(MAP.query("<Down>"), Some(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE)));
    assert_eq!(MAP.query_code("Down"), Some(KeyCode::Down));
    assert_eq!(MAP.query("<Enter>"), Some(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE)));
    assert_eq!(MAP.query_code("Enter"), Some(KeyCode::Enter));
    assert_eq!(MAP.query("<Space>"), Some(KeyEvent::new(KeyCode::Char(' '), KeyModifiers::NONE)));
    assert_eq!(MAP.query_code("Space"), Some(KeyCode::Char(' ')));
    assert_eq!(MAP.query("<BS>"), Some(KeyEvent::new(KeyCode::Backspace, KeyModifiers::NONE)));
    assert_eq!(MAP.query_code("BS"), Some(KeyCode::Backspace));
    assert_eq!(MAP.query("<Esc>"), Some(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE)));
    assert_eq!(MAP.query_code("Esc"), Some(KeyCode::Esc));
}

#[derive(Clone, Copy, PartialEq)]
enum State {
   Start, C, A, S, C_, A_, S_, Accept, Reject
}

enum Transition {
    C, A, S, Hyphen, Else
}

/// Struct that represents key press to context mapping.
pub struct Config {
    #[doc(hidden)]
    map: HashMap<String, HashMap<KeyEvent, Factory>>
}

impl Config {
    /// Create a Config from a string representing the text of the configuration.
    pub fn new(text: &str) -> Result<Config, ConfigParseError> {
        let mut map = HashMap::new();
        for (line, line_no) in text.lines().zip(0u16..) {
            let (context, keypress, factory) = Self::parse_line(line, line_no)?;
            map.entry(context).or_insert(HashMap::new())
               .entry(keypress).or_insert(factory);
        }
        Ok(Config{ map })
    }

    /// Create an empty Config.
    pub fn empty() -> Config {
        Config{ map: HashMap::new() }
    }

    /// Create a Config from a file.
    pub fn from_file(filename: PathBuf) -> Result<Config, ConfigParseError> {
        let text = read_to_string(filename)?;
        Self::new(&text)
    }

    /// Query the configuration for a context [`Factory`].
    pub fn query(&self, context: &str, key: KeyEvent) -> Option<&Factory> {
        let code = if let KeyCode::Char(c) = key.code { KeyCode::Char(c.to_ascii_uppercase()) } else { key.code };
        let key = KeyEvent::new(code, key.modifiers);
        self.map.get(context).map_or(None, |m| m.get(&key)) 
    }

    #[doc(hidden)]
    pub fn parse_key_event(key_event: &str, line_no: u16) -> Result<KeyEvent, ConfigParseError> {
        let no_modifiers = MAP.query(key_event);
        let key_event = if let Some(key) = no_modifiers { key } else {
            if key_event.get(0..1) != Some("<") || key_event.get(key_event.len() - 1..) != Some(">") {
                return Err(ConfigParseError::bind(BindParseError::MalformedKeyEventTerm, line_no));
            }
            let mut modifiers = KeyModifiers::empty();
            if let Some(key_event) = key_event.get(1..key_event.len() - 1) {
                let mut state = State::Start;
                let transitions = [[State::C, State::Reject, State::Reject, State::Reject, State::Accept, State::Accept, State::Accept, State::Accept, State::Reject],
                                   [State::A, State::Reject, State::Reject, State::Reject, State::A, State::Accept, State::A, State::Accept, State::Reject],
                                   [State::S, State::Reject, State::Reject, State::Reject, State::S, State::Accept, State::Accept, State::Accept, State::Reject],
                                   [State::Reject, State::C_, State::A_, State::S_, State::Accept, State::Accept, State::Accept, State::Accept, State::Reject],
                                   [State::Reject, State::Reject, State::Reject, State::Reject, State::Accept, State::Accept, State::Accept, State::Accept, State::Reject]];
                let mut key = String::with_capacity(8);
                for ch in key_event.chars() {
                    let transition = match ch {
                        'C' | 'c' => Transition::C,
                        'A' | 'a' => Transition::A,
                        'S' | 's' => Transition::S,
                        '-'       => Transition::Hyphen,
                        _         => Transition::Else,
                    };
                    state = transitions[transition as usize][state as usize];
                    if state == State::Accept || state == State::Reject {
                        if key.len() == 8 {
                            return Err(ConfigParseError::bind(BindParseError::MalformedKeyEventTerm, line_no));
                        }
                        key.push(ch); 
                    }
                    modifiers.insert(match state {
                        State::C => KeyModifiers::CONTROL,
                        State::A => KeyModifiers::ALT,
                        State::S => KeyModifiers::SHIFT,
                        _        => KeyModifiers::NONE
                    });
                }
                match state {
                    State::Start | State::C_ | State::A_ | State::S_ => return Err(ConfigParseError::bind(BindParseError::MalformedKeyEventTerm, line_no)),
                    State::C => {
                        key.push('C');
                        modifiers.remove(KeyModifiers::CONTROL);
                    },
                    State::A => {
                        key.push('A');
                        modifiers.remove(KeyModifiers::ALT);
                    },
                    State::S => {
                        key.push('S');
                        modifiers.remove(KeyModifiers::SHIFT);
                    },
                    _ => (),
                }
                let code = match MAP.query_code(&key) {
                    Some(c) => c,
                    None => return Err(ConfigParseError::bind(BindParseError::MalformedKeyEventTerm, line_no))
                };
                KeyEvent::new(code, modifiers)
            } else {
                return Err(ConfigParseError::bind(BindParseError::UnicodeBoundaryErrorInKeyEvent, line_no));
            }
        };
        Ok(key_event)
    }

    #[doc(hidden)]
    pub fn parse_line(line: &str, line_no: u16) -> Result<(String, KeyEvent, Factory), ConfigParseError> {
        let mut iter = line.split(' ');
        let bind = iter.next();
        let key_event = iter.next();
        let new_context = iter.next();
        if bind.is_none() || key_event.is_none() || new_context.is_none() {
            return Err(ConfigParseError::bind(BindParseError::NotEnoughTerms, line_no));
        }
        let bind = bind.unwrap();
        let key_event = key_event.unwrap();
        let new_context = new_context.unwrap();
        if bind.len() < 6 || bind.get(0..5) != Some("bind(") || bind.get(bind.len() - 1..) != Some(")") {
            return Err(ConfigParseError::bind(BindParseError::MalformedBindTerm, line_no));
        }
        if let Some(old_context) = bind.get(5..bind.len() - 1) {
            let key_event = Self::parse_key_event(key_event, line_no)?;
            let args = String::from(iter.fold(String::new(), |acc, x| acc + " " + x).trim());
           
            if let Some(factory) = context(new_context, args) {
                Ok((old_context.to_string(), key_event, factory))
            } else {
                Err(ConfigParseError::bind(BindParseError::NoMatchingContext{ context: new_context.to_string() }, line_no))
            }
        } else {
            Err(ConfigParseError::bind(BindParseError::UnicodeBoundaryErrorInBind, line_no))
        }
    }
}

#[test]
fn test_parse_line_key_event() {
    assert_eq!(Config::parse_key_event("", 1).err(), Some(ConfigParseError::bind(BindParseError::MalformedKeyEventTerm, 1)));
    assert_eq!(Config::parse_key_event("a", 1).unwrap(), KeyEvent::new(KeyCode::Char('A'), KeyModifiers::NONE));
    assert_eq!(Config::parse_key_event("B", 1).unwrap(), KeyEvent::new(KeyCode::Char('B'), KeyModifiers::NONE));
    assert_eq!(Config::parse_key_event("<Tab>", 1).unwrap(), KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE));
    assert_eq!(Config::parse_key_event("<tab>", 1).err(), Some(ConfigParseError::bind(BindParseError::MalformedKeyEventTerm, 1)));
    assert_eq!(Config::parse_key_event("<C-C>", 1).unwrap(), KeyEvent::new(KeyCode::Char('C'), KeyModifiers::CONTROL));
    assert_eq!(Config::parse_key_event("<C-S-A>", 1).unwrap(), KeyEvent::new(KeyCode::Char('A'), KeyModifiers::CONTROL.union(KeyModifiers::SHIFT)));
    assert_eq!(Config::parse_key_event("<C-A-->", 1).unwrap(), KeyEvent::new(KeyCode::Char('-'), KeyModifiers::CONTROL.union(KeyModifiers::ALT)));
    assert_eq!(Config::parse_key_event("<A-Enter>", 1).unwrap(), KeyEvent::new(KeyCode::Enter, KeyModifiers::ALT));
    assert_eq!(Config::parse_key_event("<S-V>", 1).unwrap(), KeyEvent::new(KeyCode::Char('V'), KeyModifiers::SHIFT));
    assert_eq!(Config::parse_key_event("<S-A-C>", 1).unwrap(), KeyEvent::new(KeyCode::Char('C'), KeyModifiers::SHIFT.union(KeyModifiers::ALT)));
    assert_eq!(Config::parse_key_event("<C-S-A-Del>", 1).unwrap(), KeyEvent::new(KeyCode::Delete, KeyModifiers::CONTROL.union(KeyModifiers::SHIFT.union(KeyModifiers::ALT))));
}
