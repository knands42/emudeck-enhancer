mod downloader;

use scraper::error::SelectorErrorKind;
use url::ParseError;

pub use downloader::download_texture;

#[derive(Debug)]
pub enum TextureDownloaderError {
    Parse(String),
}

impl From<reqwest::Error> for TextureDownloaderError {
    fn from(e: reqwest::Error) -> Self {
        TextureDownloaderError::Parse(e.to_string())
    }
}

impl From<SelectorErrorKind<'_>> for TextureDownloaderError {
    fn from(e: SelectorErrorKind) -> Self {
        TextureDownloaderError::Parse(e.to_string())
    }
}

impl From<ParseError> for TextureDownloaderError {
    fn from(e: ParseError) -> Self {
        TextureDownloaderError::Parse(e.to_string())
    }
}
