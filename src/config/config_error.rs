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

impl fmt::Display for BindParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { 
        match self {
            Self::NoMatchingContext{ context } => write!(f, "no matching context {} found", context),
            Self::NotEnoughTerms => write!(f, "not enough terms (expected at least 3)"),
            Self::MalformedBindTerm => write!(f, "incorrect syntax in bind term"),
            Self::UnicodeBoundaryErrorInBind => write!(f, "unexpected unicode character in bind term"),
            Self::MalformedKeyEventTerm => write!(f, "incorrect syntax in key event term"),
            Self::UnicodeBoundaryErrorInKeyEvent => write!(f, "unexpected unicode character in key event term"),
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

impl fmt::Display for IOError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// Enum for containing errors that might occur in parsing custom layout specifications.
#[derive(Debug, PartialEq)]
pub enum LayoutParseError {
    /// No first line to parse.
    NoFirstLine,
    /// The spec didn't start with a layout name.
    NoLayoutName,
    /// Non-ASCII character in layout pair.
    NonAsciiCharacter{ line: usize },
    /// No ` => ` found in layout pair.
    MalformedLayoutPair{ line: usize },
    /// Not mapping a character to a character.
    NonCharacterMapping{ line: usize },
    /// IO error (e.g. cannot open the layout file)
    IOError{ error: IOError }
}

impl fmt::Display for LayoutParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoFirstLine => write!(f, "no first line to parse"),
            Self::NoLayoutName => write!(f, "no layout name found"),
            Self::NonAsciiCharacter{ line } => write!(f, "non-ASCII character found in layout pair on line {}", line),
            Self::MalformedLayoutPair{ line } => write!(f, "did not find ` => ` in layout pair on line {}", line),
            Self::NonCharacterMapping{ line } => write!(f, "layout pair on line {} not mapping a character to a character", line),
            Self::IOError{ error } => error.fmt(f)
        }
    }
}

impl From<std::io::Error> for LayoutParseError {
    fn from(error: std::io::Error) -> Self {
        let error = IOError(error);
        Self::IOError{ error }
    }
}

/// Enum for containing errors that might occur in parsing include statements.
#[derive(Debug, PartialEq)]
pub enum IncludeParseError {
    /// No `include ` found at beginning of line.
    MalformedInclude,
    /// Neither a layout include nor config include.
    UnknownIncludeType,
    /// Found no single-quoted file name in layout include.
    LayoutNoQuotedFile,
    /// No ` as ` found in non-empty string after final single quote in layout include.
    MalformedAsClause,
}

impl fmt::Display for IncludeParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MalformedInclude => write!(f, "no `include ` found"),
            Self::UnknownIncludeType => write!(f, "neither a layout include nor a config include"),
            Self::LayoutNoQuotedFile => write!(f, "in layout include, found no single-quoted file name"),
            Self::MalformedAsClause => write!(f, "in layout include, found no ` as ` in non-empty string after final single quote"),
        }
    }
}

#[derive(Debug, PartialEq)]
/// Enum for containing errors that might occur in parsing configurations.
pub enum ConfigParseError {
    /// See [`BindParseError`].
    BindParseError{ error: BindParseError, line: usize },
    /// See [`OptionParseError`](super::options::OptionParseError).
    OptionParseError{ error: OptionParseError, line: usize },
    /// See [`LayoutParseError`].
    LayoutParseError{ error: LayoutParseError, line: usize },
    /// See [`IncludeParseError`].
    IncludeParseError{ error: IncludeParseError, line: usize },
    /// Could not determine the statement type of the line.
    NotAStatement{ line: usize },
    /// Can not set layout to unknown custom layout
    NoMatchingLayout{ line: usize },
    /// IO error (e.g. cannot open the config file)
    IOError{ error: IOError },
}

impl ConfigParseError {
    /// Create a `ConfigParseError::BindParseError` from the inner `BindParseError`.
    pub fn bind(error: BindParseError, line: usize) -> Self {
        Self::BindParseError{ error, line }
    }

    /// Create a `ConfigParseError::OptionParseError` from the inner `OptionParseError`.
    pub fn option(error: OptionParseError, line: usize) -> Self {
        Self::OptionParseError{ error, line }
    }

    /// Create a `ConfigParseError::LayoutParseError` from the inner `LayoutParseError`.
    pub fn layout(error: LayoutParseError, line: usize) -> Self {
        Self::LayoutParseError{ error, line } 
    }

    /// Create a `ConfigParseError::IncludeParseError` from the inner `IncludeParseError`.
    pub fn include(error: IncludeParseError, line: usize) -> Self {
        Self::IncludeParseError{ error, line }
    }
}

impl fmt::Display for ConfigParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BindParseError{ error, line } => write!(f, "error parsing bind statement on line {}: {}", line, error),
            Self::OptionParseError{ error, line } => write!(f, "error parsing option statement on line {}: {}", line, error),
            Self::LayoutParseError{ error, line } => write!(f, "error parsing layout spec (included on line {}): {}", line, error),
            Self::IncludeParseError{ error, line } => write!(f, "error parsing include statement on line {}: {}", line, error),
            Self::NotAStatement{ line } => write!(f, "could not determine statement type of line {}", line),
            Self::NoMatchingLayout{ line } => write!(f, "could not find an included custom layout matching line {}", line),
            Self::IOError{ error } => error.fmt(f),
        }
    }
}

impl From<std::io::Error> for ConfigParseError {
    fn from(e: std::io::Error) -> Self {
        Self::IOError{ error: IOError(e) }
    }
}

impl std::error::Error for ConfigParseError {}
