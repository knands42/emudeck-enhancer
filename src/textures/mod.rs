mod downloader;
mod manager;

use scraper::error::SelectorErrorKind;
use url::ParseError;

pub use manager::TextureManager;

#[derive(Debug)]
pub enum TextureError {
    Parse(String),
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
