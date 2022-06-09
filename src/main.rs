//! This is a vim-like editor that provides support for multiple keyboard layouts, like Dvorak and
//! Colemak. It is not intended to be a vim clone, but to provide many similar commands and
//! functions, while building off of other features.
//!
//! (vimscript or its analogue won't be included for a long time, sorry)

pub use libfim::{config, context, document, editor, layout, terminal, window};
use libfim::config::keybinds::Config;
use libfim::config::options::Options;
use libfim::editor::Editor;
use clap::Parser;
use std::path::PathBuf;

#[doc(hidden)]
#[derive(Parser)]
#[clap(version)]
#[clap(author = "Carson Foster")]
#[clap(about = "vim-like text editor with support for multiple keyboard layouts.", long_about = None)]
struct Args {
    /// File to edit
    #[clap(parse(from_os_str), value_name = "FILE")]
    file: Option<PathBuf>,
    /// Path to configuration file for fim
    #[clap(short = 'u', parse(from_os_str), value_name = "CONFIG_FILE")]
    config_file: Option<PathBuf>,
}

#[doc(hidden)]
fn main() {
    let mut args = Args::parse();
    let opt = Options::default(); // TODO: options
    let config = if let Some(config) = args.config_file {
        let filename = config.as_os_str().to_string_lossy().to_string();
        let result = Config::from_file(config);
        if let Err(e) = result {
            println!("[-] Failed to parse configuration file {}: {}", filename, e);
            std::process::exit(1);
        }
        Some(result.unwrap())
    } else { None };
    let fim = if let Some(filename) = args.file.take() {
        Editor::new(filename, opt, config)
    } else {
        Editor::default(opt, config)
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
