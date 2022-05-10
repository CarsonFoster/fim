//! A module for handling configuration files that map keyboard presses to contexts.
//!
//! # Config Format
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
//! # Key Event Format
//! Undecided as of yet.
use crate::context::*;
use std::fmt;
use std::fs::read_to_string;
use std::collections::HashMap;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

#[derive(Debug)]
#[non_exhaustive]
/// Enum for containing errors that might occur in parsing configurations.
pub enum ConfigParseError {
    /// User wants to map a key to a non-existent context.
    NoMatchingContext{ context: String, line: u16 }, 
    /// Not enough terms in a `bind` line.
    NotEnoughTerms{ line: u16 },
    /// The `bind` term isn't formed correctly.
    MalformedBindTerm{ line: u16 },
    /// Unexpected unicode character in the `bind` term.
    UnicodeBoundaryErrorInBind{ line: u16 },
    /// The key event term isn't formed correctly.
    MalformedKeyEventTerm{ line: u16 },
    /// Unexpected unicode character in the key event term.
    UnicodeBoundaryErrorInKeyEvent{ line: u16 },
    /// IO error (e.g. cannot open the config file)
    IOError{ error: std::io::Error },
}

#[doc(hidden)]
impl ConfigParseError {
    pub fn value(&self) -> String {
        match self {
            ConfigParseError::NoMatchingContext{ context, line } => format!("line {}: no matching context {} found", line, context),
            ConfigParseError::NotEnoughTerms{ line } => format!("line {}: not enough terms (expected at least 3)", line),
            ConfigParseError::MalformedBindTerm{ line } => format!("line {}: incorrect syntax in bind term", line),
            ConfigParseError::UnicodeBoundaryErrorInBind{ line } => format!("line {}: unexpected unicode character in bind term", line),
            ConfigParseError::MalformedKeyEventTerm{ line } => format!("line {}: incorrect syntax in key event term", line),
            ConfigParseError::UnicodeBoundaryErrorInKeyEvent{ line } => format!("line {}: unexpected unicode character in key event term", line),
            ConfigParseError::IOError{ error } => error.to_string(),
            _ => "unknown config parse error".to_owned(),
        }
    }
}

impl fmt::Display for ConfigParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigParseError::IOError{ error } => error.fmt(f),
            _ => write!(f, "Error in parsing configuration: {}", self.value()),
        }
    }
}

impl From<std::io::Error> for ConfigParseError {
    fn from(e: std::io::Error) -> Self {
        ConfigParseError::IOError{ error: e }
    }
}

impl std::error::Error for ConfigParseError {}

struct ConfigMap {
    unprintable: HashMap<String, KeyEvent>,
}

impl ConfigMap {
    fn new() -> Self {
        let mut unprintable = HashMap::new();
        let pairs = [("BS", KeyCode::Backspace), ("CR", KeyCode::Enter), ("Enter", KeyCode::Enter), ("Left", KeyCode::Left),
                     ("Right", KeyCode::Right), ("Up", KeyCode::Up), ("Down", KeyCode::Down), ("Home", KeyCode::Home),
                     ("End", KeyCode::End), ("PageUp", KeyCode::PageUp), ("PageDown", KeyCode::PageDown), ("Tab", KeyCode::Tab),
                     ("Del", KeyCode::Delete), ("Ins", KeyCode::Insert)];
        for (rep, code) in pairs {
            unprintable.insert(rep.to_string(), KeyEvent::new(code, KeyModifiers::NONE));
        }
        for i in 1u8..=12 {
            unprintable.insert(format!("F{}", i), KeyEvent::new(KeyCode::F(i), KeyModifiers::NONE));
        }
        ConfigMap{ unprintable }
    }

    pub fn query(&self, rep: &str) -> Option<KeyEvent> {
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

#[derive(Clone, Copy, PartialEq)]
enum State {
   Start, C, A, S, C_, A_, S_, Accept, Reject
}

enum Transition {
    C, A, S, Hyphen, Else
}

/// Struct that represents key press to context mapping.
pub struct Config {
    map: HashMap<String, HashMap<KeyEvent, Box<dyn Fn() -> Box<dyn Context>>>>
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

    /// Create a Config from a file.
    pub fn from_file(filename: &str) -> Result<Config, ConfigParseError> {
        let text = read_to_string(filename)?;
        Self::new(&text)
    }

    fn parse_line(line: &str, line_no: u16) -> Result<(String, KeyEvent, Box<dyn Fn() -> Box<dyn Context>>), ConfigParseError> {
        let mut iter = line.split(' ');
        let bind = iter.next();
        let key_event = iter.next();
        let new_context = iter.next();
        if bind.is_none() || key_event.is_none() || new_context.is_none() {
            return Err(ConfigParseError::NotEnoughTerms{ line: line_no });
        }
        let bind = bind.unwrap();
        let key_event = key_event.unwrap();
        let new_context = new_context.unwrap();
        if bind.len() < 6 || bind.get(0..5) != Some("bind(") || bind.get(bind.len() - 1..) != Some(")") {
            return Err(ConfigParseError::MalformedBindTerm{ line: line_no });
        }
        if let Some(old_context) = bind.get(5..bind.len() - 1) {
            let single_key = MAP.query(key_event);
            let key_event = if let Some(key) = single_key { key } else {
                if key_event.get(0..1) != Some("<") || key_event.get(key_event.len() - 1..) != Some(">") {
                    return Err(ConfigParseError::MalformedKeyEventTerm{ line: line_no });
                }
                let mut modifiers = KeyModifiers::empty();
                if let Some(key_event) = key_event.get(1..key_event.len() - 1) {
                    let mut state = State::Start;
                    let transitions = [[State::C, State::Reject, State::Reject, State::Reject, State::Accept, State::Accept, State::Accept],
                                       [State::A, State::Reject, State::Reject, State::Reject, State::A, State::Accept, State::A],
                                       [State::S, State::Reject, State::Reject, State::Reject, State::S, State::Accept, State::Accept],
                                       [State::Reject, State::C_, State::A_, State::S_, State::Accept, State::Accept, State::Accept],
                                       [State::Reject, State::Reject, State::Reject, State::Reject, State::Accept, State::Accept, State::Accept]];
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
                        if state == State::Reject {
                            return Err(ConfigParseError::MalformedKeyEventTerm{ line: line_no });
                        } else if state == State::Accept {
                            if key.len() == 8 {
                                return Err(ConfigParseError::MalformedKeyEventTerm{ line: line_no });
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
                    let code = match MAP.query_code(&key) {
                        Some(c) => c,
                        None => return Err(ConfigParseError::MalformedKeyEventTerm{ line: line_no })
                    };
                    KeyEvent::new(code, modifiers)
                } else {
                    return Err(ConfigParseError::UnicodeBoundaryErrorInKeyEvent{ line: line_no });
                }
            };
            // TODO: get args
            let args = iter.fold(String::new(), |acc, x| acc + " " + x);
           
            if let Some(factory) = context(new_context, args) {
                Ok((old_context.to_string(), key_event, factory))
            } else {
                Err(ConfigParseError::NoMatchingContext{ context: new_context.to_string(), line: line_no })
            }
        } else {
            Err(ConfigParseError::UnicodeBoundaryErrorInBind{ line: line_no })
        }
    }
}
