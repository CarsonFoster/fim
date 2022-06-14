//! A module that provides the error types for configuration parsing.

pub use super::options::OptionParseError;
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
            Self::NoMatchingContext{ context } => format!("no matching context {} found", context),
            Self::NotEnoughTerms => format!("not enough terms (expected at least 3)"),
            Self::MalformedBindTerm => format!("incorrect syntax in bind term"),
            Self::UnicodeBoundaryErrorInBind => format!("unexpected unicode character in bind term"),
            Self::MalformedKeyEventTerm => format!("incorrect syntax in key event term"),
            Self::UnicodeBoundaryErrorInKeyEvent => format!("unexpected unicode character in key event term"),
        }
    }
}

/// Newtype on [`std::io::Error`] to give it PartialEq by kind.
#[derive(Debug)]
pub struct IOError(std::io::Error);

impl PartialEq for IOError {
    fn eq(&self, other: &Self) -> bool {
        self.0.kind() == other.0.kind()
    }
}

/// Enum for containing errors that might occur in parsing custom layout specifications.
#[derive(Debug, PartialEq)]
pub enum LayoutParseError {
    /// IO error (e.g. cannot open the layout file)
    IOError{ error: IOError },
}

#[derive(Debug, PartialEq)]
/// Enum for containing errors that might occur in parsing configurations.
pub enum ConfigParseError {
    /// See [`BindParseError`].
    BindParseError{ error: BindParseError, line: u16 },
    /// See [`OptionParseError`](super::options::OptionParseError).
    OptionParseError{ error: OptionParseError, line: u16 },
    /// Could not determine the statement type of the line.
    NotAStatement{ line: u16 },
    /// IO error (e.g. cannot open the config file)
    IOError{ error: IOError },
}

impl ConfigParseError {
    /// Create a `ConfigParseError::BindParseError` from the inner `BindParseError`.
    pub fn bind(error: BindParseError, line: u16) -> Self {
        Self::BindParseError{ error, line }
    }

    /// Create a `ConfigParseError::OptionParseError` from the inner `OptionParseError`.
    pub fn option(error: OptionParseError, line: u16) -> Self {
        Self::OptionParseError{ error, line }
    }

    #[doc(hidden)]
    pub fn value(&self) -> String {
        match self {
            Self::BindParseError{ error, line } => format!("error parsing bind statement on line {}: {}", line, error.value()),
            Self::OptionParseError{ error, line } => format!("error parsing option statement on line {}: {}", line, error.value()),
            Self::NotAStatement{ line } => format!("could not determine statement type of line {}", line),
            Self::IOError{ error } => error.0.to_string(),
        }
    }
}

impl fmt::Display for ConfigParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IOError{ error } => error.0.fmt(f),
            _ => write!(f, "error in parsing configuration: {}", self.value()),
        }
    }
}

impl From<std::io::Error> for ConfigParseError {
    fn from(e: std::io::Error) -> Self {
        Self::IOError{ error: IOError(e) }
    }
}

impl std::error::Error for ConfigParseError {}
