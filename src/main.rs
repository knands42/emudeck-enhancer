mod extractor;

use scraper::{Html, Selector, error::SelectorErrorKind};
use url::{Url, ParseError};


#[derive(Debug)]
enum AppError {
    Http(reqwest::Error),
    Parse(String),
    Extractor(extractor::ExtractorError)
}

impl From<extractor::ExtractorError> for AppError {
    fn from(e: extractor::ExtractorError) -> Self {
        AppError::Extractor(e)
    }
}

impl From<reqwest::Error> for AppError {
    fn from(e: reqwest::Error) -> Self {
        AppError::Parse(e.to_string())
    }
}

impl From<SelectorErrorKind<'_>> for AppError {
    fn from(e: SelectorErrorKind) -> Self {
        AppError::Parse(e.to_string())
    }
}

impl From<ParseError> for AppError {
    fn from(e: ParseError) -> Self {
        AppError::Parse(e.to_string())
    }
}


#[tokio::main]
async fn main() -> Result<(), AppError> {
    let game_info = extractor::extract("");
    Ok(())
}