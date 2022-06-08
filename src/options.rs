//! A module for handling internal configuration options.
//!
//! There are three different types of options: boolean, numeric, and string. These can be set by
//! the user through configuration files or in-fim commands (eventually, not right now).

use read_option::ReadOption;
use option_string::OptionString;

/// Struct that represent the collection of internal configuration options.
#[derive(Clone, ReadOption)]
pub struct Options {
    /// Type of line numbering to use; string (enum) option
    pub line_numbering: LineNumbers,
    /// Keyboard layout to use; string option
    pub layout: LayoutType,
}

/// The defaults are relative line numbering and the QWERTY layout.
impl Default for Options {
    fn default() -> Self {
        Options{ line_numbering: LineNumbers::Relative, layout: LayoutType::Qwerty }
    }
}

/// Struct that creates an [`Options`] object.
pub struct OptionFactory {
    #[doc(hidden)]
    opt: Options,
}

impl OptionFactory {
    /// Create a new [`OptionFactory`].
    ///
    /// The `Options` object begins with its default value.
    pub fn new() -> Self {
        OptionFactory{ opt: Options::default() }
    }
     
    /// Consume the `OptionFactory` and return the created `Options` object.
    pub fn options(self) -> Options {
        self.opt
    }

    /// Return a reference to the `Options` object in the process of being created.
    pub fn peek(&self) -> &Options {
        &self.opt
    }

    /// Set the `line_numbering` field of the `Options` object.
    pub fn set_line_numbering(&mut self, numbering: LineNumbers) -> &mut Self {
        self.opt.line_numbering = numbering; 
        self
    }

    /// Set the `layout` field of the `Options` object.
    pub fn set_layout(&mut self, layout: LayoutType) -> &mut Self {
        self.opt.layout = layout;
        self
    }
}

#[derive(Copy, Clone, OptionString)]
/// Enum that represents the different types of line numbers fim can use.
///
/// String (enum) option: possible values are `Off`, `On`, and `Relative`.
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
#[derive(Clone)]
pub enum LayoutType {
    /// The standard keyboard layout.
    Qwerty,
    /// The [Dvorak](https://en.wikipedia.org/wiki/Dvorak_keyboard_layout) keyboard layout.
    Dvorak,
    /// The [Colemak](https://en.wikipedia.org/wiki/Colemak) keyboard layout.
    Colemak,
    /// A user-defined keyboard layout.
    Custom {
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
