//! A module for handling configuration files that map keyboard presses to contexts.
//!
//! # Config Format
//! Undecided, as of yet.
use std::fmt;
use std::fs::read_to_string;

#[derive(Debug)]
#[non_exhaustive]
/// Enum for containing errors that might occur in parsing configurations.
pub enum ConfigParseError {
    /// User wants to map a key to a non-existent context.
    NoMatchingContext{ context: String, line: u16 }, 
    /// IO error (e.g. cannot open the config file)
    IOError{ error: std::io::Error },
}

#[doc(hidden)]
impl ConfigParseError {
    pub fn value(&self) -> String {
        match self {
            ConfigParseError::NoMatchingContext{ context, line } => format!("line {}: no matching context {} found", line, context),
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

}

impl Config {
    /// Create a Config from a string representing the text of the configuration.
    pub fn new(text: &str) -> Result<Config, ConfigParseError> {
        Ok(Config{ })
    }

    /// Create a Config from a file.
    pub fn from_file(filename: &str) -> Result<Config, ConfigParseError> {
        let text = read_to_string(filename)?;
        Self::new(&text)
    }
}
