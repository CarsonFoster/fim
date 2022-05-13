//! This is a vim-like editor that provides support for multiple keyboard layouts, like Dvorak and
//! Colemak. It is not intended to be a vim clone, but to provide many similar commands and
//! functions, while building off of other features.
//!
//! (vimscript or its analogue won't be included for a long time, sorry)

pub use libfim::{config, context, document, editor, layout, terminal, window};
use libfim::editor::Editor;

#[doc(hidden)]
fn main() {
    match Editor::default() {
        Ok(mut fim) => {
            if let Err(e) = fim.run() {
                std::mem::drop(fim);
                println!("[-] Application error: {}", e);
                std::process::exit(1);
            }
        },
        Err(e) => {
            println!("[-] Failed to initialize editor: {}", e);
            std::process::exit(1);
        }
    }
}
