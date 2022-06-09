//! A module that provides the error types for configuration parsing.
use std::fmt;

#[derive(Debug, PartialEq)]
/// Enum for containing errors that might occur in parsing bind lines.
pub enum BindParseError {
    /// User wants to map a key to a non-existent context.
    NoMatchingContext{ context: String }, 
    /// Not enough terms in a `bind` line.
    NotEnoughTerms,
    /// The `bind` term isn't formed correctly.
    MalformedBindTerm,
    /// Unexpected unicode character in the `bind` term.
    UnicodeBoundaryErrorInBind,
    /// The key event term isn't formed correctly.
    MalformedKeyEventTerm,
    /// Unexpected unicode character in the key event term.
    UnicodeBoundaryErrorInKeyEvent,
}

#[doc(hidden)]
impl BindParseError {
    pub fn value(&self) -> String { 
        match self {
            BindParseError::NoMatchingContext{ context } => format!("no matching context {} found", context),
            BindParseError::NotEnoughTerms => format!("not enough terms (expected at least 3)"),
            BindParseError::MalformedBindTerm => format!("incorrect syntax in bind term"),
            BindParseError::UnicodeBoundaryErrorInBind => format!("unexpected unicode character in bind term"),
            BindParseError::MalformedKeyEventTerm => format!("incorrect syntax in key event term"),
            BindParseError::UnicodeBoundaryErrorInKeyEvent => format!("unexpected unicode character in key event term"),
        }
    }
}

#[derive(Debug)]
/// Enum for containing errors that might occur in parsing configurations.
pub enum ConfigParseError {
    /// See [`BindParseError`].
    BindParseError{ error: BindParseError, line: u16 },
    /// IO error (e.g. cannot open the config file)
    IOError{ error: std::io::Error },
}

impl ConfigParseError {
    /// Create a `ConfigParseError::BindParseError` from the inner BindParseError.
    pub fn bind(error: BindParseError, line: u16) -> Self {
        ConfigParseError::BindParseError{ error, line }
    }

    #[doc(hidden)]
    pub fn value(&self) -> String {
        match self {
            ConfigParseError::BindParseError{ error, line } => format!("error parsing bind statement on line {}: {}", line, error.value()),
            ConfigParseError::IOError{ error } => error.to_string(),
        }
    }
}

impl PartialEq for ConfigParseError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) { 
            (ConfigParseError::BindParseError{ error, line }, ConfigParseError::BindParseError{ error: other_error, line: other_line })
                => line == other_line && error == other_error,
            (ConfigParseError::IOError{ error }, ConfigParseError::IOError{ error: other_error })
                => error.kind() == other_error.kind(),
            _ => false
        }
    }
}

impl fmt::Display for ConfigParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigParseError::IOError{ error } => error.fmt(f),
            _ => write!(f, "error in parsing configuration: {}", self.value()),
        }
    }
}

impl From<std::io::Error> for ConfigParseError {
    fn from(e: std::io::Error) -> Self {
        ConfigParseError::IOError{ error: e }
    }
}

impl std::error::Error for ConfigParseError {}
