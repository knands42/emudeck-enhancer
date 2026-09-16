mod listen;
pub use listen::listen_to;

use std::path::PathBuf;

pub struct GameInfo {
    pub path: PathBuf,
    pub name: String,
    pub serial: Option<String>,
}

impl<'a> GameInfo {
    pub fn new(path: PathBuf, name: String, serial: Option<String>) -> Self {
        Self { path, name, serial }
    }
}
