mod listener;

use scraper::{Html, Selector, error::SelectorErrorKind};
use url::{Url, ParseError};



#[derive(Debug)]
enum AppError {
    Http(reqwest::Error),
    Parse(String)
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
    listener::listen_to("");
    Ok(())
}
