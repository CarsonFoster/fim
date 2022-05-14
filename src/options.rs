//! A module for handling internal configuration options.
//!
//! There are three different types of options: boolean, numeric, and string. These can be set by
//! the user through configuration files or in-fim commands (eventually, not right now).

/// Struct that represent the collection of internal configuration options.
pub struct Options {
    pub line_numbering: LineNumbers,
}

impl Default for Options {
    fn default() -> Self {
        Options{ line_numbering: LineNumbers::Relative }
    }
}

/// Struct that creates an [`Options`] object.
pub struct OptionFactory {
    #[doc(hidden)]
    opt: Options,
}

impl OptionFactory {
    /// Create a new [`OptionFactory`].
    pub fn new() -> Self {
        OptionFactory{ opt: Options::default() }
    }
     
    /// Consume the `OptionFactory` and return the created `Options` object.
    pub fn options(self) -> Options {
        self.opt
    }

    /// Set the `line_numbering` field of the `Options` object.
    pub fn set_line_numbering(&mut self, numbering: LineNumbers) -> &mut Self {
        self.opt.line_numbering = numbering; 
        self
    }
}

/// Enum that represents the different types of line numbers fim can use.
pub enum LineNumbers {
    Off,
    On,
    Relative
}
