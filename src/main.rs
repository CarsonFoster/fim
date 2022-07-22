//! This is a vim-like editor that provides support for multiple keyboard layouts, like Dvorak and
//! Colemak. It is not intended to be a vim clone, but to provide many similar commands and
//! functions, while building off of other features.
//!
//! (vimscript or its analogue won't be included for a long time, sorry)

pub use libfim::{config, context, delta, editor, grapheme_string, layout, terminal, window};
use libfim::config::Config;
use libfim::editor::Editor;
use clap::Parser;
use std::fs;
use std::io::{stderr, Write};
use std::panic;
use std::path::PathBuf;

const LOG_FILE: &str = "fim_crash.log";

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
    panic::set_hook(Box::new(|info| {
        let payload = if let Some(p) = info.payload().downcast_ref::<String>() {
            p.as_str()
        } else if let Some(p) = info.payload().downcast_ref::<&str>() {
            p
        } else {
            "non-string payload"
        };
        let message = if let Some(loc) = info.location() {
            format!("{}:{}, column {} | payload: {}\n", loc.file(), loc.line(), loc.column(), payload)
        } else {
            format!("payload: {} (no location info)\n", payload)
        };
        if fs::write(LOG_FILE, message.clone()).is_err() {
            write!(stderr(), "{}", message).ok();
        }
    }));
    let mut args = Args::parse();
    let config = if let Some(config) = args.config_file {
        let filename = config.as_os_str().to_string_lossy().to_string();
        let result = Config::new(config);
        if let Err(e) = result {
            println!("[-] Failed to parse configuration file {}: {}", filename, e);
            std::process::exit(1);
        }
        Some(result.unwrap())
    } else { None };
    let fim = if let Some(filename) = args.file.take() {
        Editor::new(filename, config)
    } else {
        Editor::default(config)
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
