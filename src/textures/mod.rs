mod downloader;
mod extractor;
mod manager;

use scraper::error::SelectorErrorKind;
use url::ParseError;

pub use manager::TextureManager;

#[derive(Debug)]
pub enum TextureError {
    Parse(String),
    Error(String),
}

impl From<reqwest::Error> for TextureError {
    fn from(e: reqwest::Error) -> Self {
        TextureError::Parse(e.to_string())
    }
}

impl From<SelectorErrorKind<'_>> for TextureError {
    fn from(e: SelectorErrorKind) -> Self {
        TextureError::Parse(e.to_string())
    }
}

impl From<ParseError> for TextureError {
    fn from(e: ParseError) -> Self {
        TextureError::Parse(e.to_string())
    }
}

impl From<std::io::Error> for TextureError {
    fn from(e: std::io::Error) -> Self {
        TextureError::Error(e.to_string())
    }
}
