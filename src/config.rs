use crate::editor::Editor;
use crossterm::{Result, event::KeyEvent};

pub trait Config {
    fn action(&self, key: KeyEvent) -> Option<dyn FnOnce(&mut Editor) -> Result<()>>;
}

pub struct ConfigFile {

}

impl ConfigFile {
    pub fn new(filename: &str) -> Self {
        ConfigFile
    }
}

pub struct DefaultConfig;

impl Config for DefaultConfig {
    fn action(&self, key: KeyEvent) -> Option<dyn FnOnce(&mut Editor) -> Result<()>> {
        match key {
             
        }
    }
}
