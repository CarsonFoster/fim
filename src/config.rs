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
use crossterm::event::KeyEvent;

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
    /// IO error (e.g. cannot open the config file)
    IOError{ error: std::io::Error },
}

#[doc(hidden)]
impl ConfigParseError {
    pub fn value(&self) -> String {
        match self {
            ConfigParseError::NoMatchingContext{ context, line } => format!("line {}: no matching context {} found", line, context),
            ConfigParseError::NotEnoughTerms{ line } => format!("line {}: not enough terms (expected at least 3)", line),
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
        let mut iter = line.split(' ').take(3);
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
            // TODO: parse key_event into KeyEvent
            // TODO: get args
           
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
