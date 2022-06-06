//! A module for handling internal configuration options.
//!
//! There are three different types of options: boolean, numeric, and string. These can be set by
//! the user through configuration files or in-fim commands (eventually, not right now).

/// Struct that represent the collection of internal configuration options.
#[derive(Clone)]
pub struct Options {
    pub line_numbering: LineNumbers,
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

#[derive(Copy, Clone)]
/// Enum that represents the different types of line numbers fim can use.
pub enum LineNumbers {
    Off,
    On,
    Relative
}

/// Enum that represents different keyboard layouts.
/// 
/// There are three built-in layouts: QWERTY, Dvorak, and Colemak. Users can also define their own
/// layouts, providing a string to identify it. 
#[derive(Clone)]
pub enum LayoutType {
    Qwerty,
    Dvorak,
    Colemak,
    Custom {
        name: String
    }
}
