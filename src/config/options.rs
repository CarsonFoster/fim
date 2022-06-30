//! A module for handling internal configuration options.
//!
//! There are three different types of options: boolean, numeric, and string. These can be set by
//! the user through configuration files or in-fim commands (eventually, not right now).

use read_option::ReadOption;
use option_factory::OptionFactory;
use option_number::OptionNumber;
use option_string::OptionString;
use std::num::ParseIntError;

/// Struct that represent the collection of internal configuration options.
#[derive(Clone, OptionFactory, ReadOption)]
pub struct Options {
    /// Type of line numbering to use; string (enum) option
    pub line_numbering: LineNumbers,
    /// Keyboard layout to use; string option
    pub layout: LayoutType,
    /// How to interpret new and existing tabs: as tab characters or as spaces.
    pub tab_type: TabType,
    /// Number of spaces to use for a tab (only when using spaces for tabs).
    pub tab_spaces: TabSpaces,
    /// Width of tab character (only when using tab for tabs).
    pub tab_width: TabWidth,
}

/// The defaults are relative line numbering and the QWERTY layout.
impl Default for Options {
    fn default() -> Self {
        Options{ line_numbering: LineNumbers::Relative, layout: LayoutType::Qwerty, tab_type: TabType::Spaces, tab_spaces: 4.into(), tab_width: 4.into() }
    }
}

/// Trait that represents a predicate to determine if a parsed `i32` is valid for this number
/// option.
pub trait Verifiable {
    /// Returns `Ok(())` if the passed `i32` is valid, and `Err(<str>)` otherwise, where `<str>` is
    /// a `String` error message.
    fn verify(_: i32) -> Result<(), String>;
}

/// Enum for containing errors that might occur in parsing number options.
#[derive(Debug, PartialEq)]
pub enum NumberParseError {
    /// The string couldn't be parsed into an integer.
    ParseIntError(ParseIntError),
    /// The integer did not pass verification.
    ///
    /// Contains the message from [`Verifiable::verify()`].
    VerificationFailed(String)
}

impl From<ParseIntError> for NumberParseError {
    fn from(e: ParseIntError) -> Self {
        Self::ParseIntError(e)
    }
}

impl From<String> for NumberParseError {
    fn from(e: String) -> Self {
        Self::VerificationFailed(e)
    }
}

/// Enum that represents the different types of line numbers fim can use.
///
/// String (enum) option: possible values are `Off`, `On`, and `Relative`.
#[derive(Copy, Clone, OptionString)]
pub enum LineNumbers {
    /// No line numbering.
    Off,
    /// Absolute line numbering.
    ///
    /// Each line is labeled with its line number.
    On,
    /// Relative line numbering.
    ///
    /// The line the cursor is on is labeled with its line number, while all other lines are
    /// labeled with how many lines away they are from the current line.
    Relative
}

/// Enum that represents different keyboard layouts.
/// 
/// There are three built-in layouts: QWERTY, Dvorak, and Colemak. Users can also define their own
/// layouts, providing a string to identify it. 
/// String option: possible values are `Qwerty`, `Dvorak`, `Colemak`, or any other string (for a
/// custom layout).
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum LayoutType {
    /// The standard keyboard layout.
    Qwerty,
    /// The [Dvorak](https://en.wikipedia.org/wiki/Dvorak_keyboard_layout) keyboard layout.
    Dvorak,
    /// The [Colemak](https://en.wikipedia.org/wiki/Colemak) keyboard layout.
    Colemak,
    /// A user-defined keyboard layout.
    Custom {
        /// The name of the custom keyboard layout.
        name: String
    }
}

impl std::str::FromStr for LayoutType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "Qwerty" => Self::Qwerty,
            "Dvorak" => Self::Dvorak,
            "Colemak" => Self::Colemak,
            _ => Self::Custom{ name: s.to_string() }
        })
    }
}

/// Enum that represents the way fim should interpret tabs.
///
/// String (enum) option: possible values are `Tab` and `Spaces`.
#[derive(Copy, Clone, OptionString)]
pub enum TabType {
    /// Use a tab character for new and existing tabs.
    Tab,
    /// Use spaces for new and existing tabs.
    Spaces
}

/// Struct that represents the width of a tab character.
///
/// This only applies when `tab_type` is `Tab`.
/// For example, you could have tab characters be 4 characters long:
/// ```no_run
/// fn main() {
///     println!("Hello, world!");
/// }
/// ```
/// 
/// Or 8 characters long:
/// ```no_run
/// fn main() {
///         println!("Hello, world!");
/// }
/// ```
#[derive(Copy, Clone, OptionNumber)]
pub struct TabWidth(i32);

impl Verifiable for TabWidth {
    fn verify(x: i32) -> Result<(), String> {
        if x > 0 { Ok(()) } else { Err("number must be positive (i.e. not negative or zero)".to_owned()) }
    }
}

/// Struct that represents the number of spaces to use for a tab character.
///
/// This only applies when `tab_type` is `Spaces`.
#[derive(Copy, Clone, OptionNumber)]
pub struct TabSpaces(i32);

impl Verifiable for TabSpaces {
    fn verify(x: i32) -> Result<(), String> {
        if x > 0 { Ok(()) } else { Err("number must be positive (i.e. not negative or zero)".to_owned()) }
    }
}
