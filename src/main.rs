//! This is a vim-like editor that provides support for multiple keyboard layouts, like Dvorak and
//! Colemak. It is not intended to be a vim clone, but to provide many similar commands and
//! functions, while building off of other features.
//!
//! (vimscript or its analogue won't be included for a long time, sorry)

pub use libfim::{config, context, document, editor, layout, options, terminal, window};
use libfim::editor::Editor;
use libfim::options::Options;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[clap(version)]
#[clap(author = "Carson Foster")]
#[clap(about = "vim-like text editor with support for multiple keyboard layouts.", long_about = None)]
struct Args {
    /// File to edit
    #[clap(parse(from_os_str), value_name = "FILE")]
    file: Option<PathBuf>,
}

#[doc(hidden)]
fn main() {
    let mut args = Args::parse();
    let opt = Options::default();
    // TODO: config file handling
    let fim = if let Some(filename) = args.file.take() {
        Editor::new(filename, opt, None)
    } else {
        Editor::default(opt, None)
    };
    match fim {
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
