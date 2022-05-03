use std::fmt;
use std::fs::read_to_string;

#[derive(Debug)]
#[non_exhaustive]
pub enum ConfigParseError {
    NoMatchingContext{ context: String, line: u16 }, 
    IOError{ error: std::io::Error },
}

impl ConfigParseError {
    pub fn value(&self) -> String {
        match self {
            ConfigParseError::NoMatchingContext{ context, line } => format!("line {}: no matching context {} found", line, context),
            ConfigParseError::IOError{ error } => error.to_string(),
            _ => "unknown config parse error".to_owned(),
        }
    }
}

impl fmt::Display for ConfigParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigParseError::IOError{ error } => error.fmt(f),
            _ => write!(f, "Error in parsing configuration: {}", self.value()),
        }
    }
}

impl From<std::io::Error> for ConfigParseError {
    fn from(e: std::io::Error) -> Self {
        ConfigParseError::IOError{ error: e }
    }
}

impl std::error::Error for ConfigParseError {}

pub struct Config {

}

impl Config {
    pub fn new(text: &str) -> Result<Config, ConfigParseError> {
        Ok(Config{ })
    }

    pub fn from_file(filename: &str) -> Result<Config, ConfigParseError> {
        let text = read_to_string(filename)?;
        Self::new(&text)
    }
}
