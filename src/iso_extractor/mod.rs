mod extract;
pub use extract::extract;

use std::path::PathBuf;

pub struct GameInfo {
    pub path: PathBuf,
    pub name: String,
    pub serial: Option<String>,
}

impl GameInfo {
    pub fn new(path: PathBuf, name: String, serial: Option<String>) -> Self {
        Self { path, name, serial }
    }
}

#[derive(Debug)]
pub enum ExtractorError {
    Io(std::io::Error),
    MissingSystemCnf,
}

impl std::fmt::Display for ExtractorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExtractorError::Io(e) => write!(f, "io error: {}", e),
            ExtractorError::MissingSystemCnf => write!(f, "could not find SYSTEM.CNF in ISO"),
        }
    }
}

impl std::error::Error for ExtractorError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ExtractorError::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for ExtractorError {
    fn from(e: std::io::Error) -> Self {
        ExtractorError::Io(e)
    }
}
