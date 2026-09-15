mod listener;

use scraper::{Html, Selector, error::SelectorErrorKind};
use url::{Url, ParseError};

async fn fetch(url: &str) -> Result<String, AppError> {
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    Ok(body)
}

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

async fn download_texture(slur: &str) -> Result<(), AppError> {
    let url = "https://archive.org/download/pcsx2-hd-texture-packs";
    let html = fetch(url).await?;

    let document = Html::parse_document(&html);
    let selector = Selector::parse("table.directory-listing-table")?;
    let row = Selector::parse("tbody tr")?;
    let link = Selector::parse("a")?;

    if let Some(table) = document.select(&selector).next() {
        for row in table.select(&row) {
            if let Some(link) = row.select(&link).next() {
                if let Some(href) = link.attr("href") {
                    let mut full_url = String::from(url);
                    full_url.push_str("/");
                    full_url.push_str(href);
                    
                    let parsed_url = Url::parse(full_url.as_str())?;
                    println!("URL: {}", parsed_url);
                }
            }
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    Ok(())
}
