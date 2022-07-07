#![warn(missing_docs)]
//! There's nothing in the crate root really, this is just an organizer for all of the modules.

#[macro_use]
extern crate lazy_static;

pub mod config;
pub mod context;
pub mod delta;
pub mod document;
pub mod editor;
pub mod grapheme_string;
pub mod layout;
pub mod terminal;
pub mod window;
