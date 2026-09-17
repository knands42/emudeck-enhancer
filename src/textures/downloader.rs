use std::path::PathBuf;

use scraper::{Html, Selector};
use url::Url;

use crate::textures::TextureError;

async fn fetch(url: &str) -> Result<String, TextureError> {
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    Ok(body)
}

pub async fn download_texture(slur: &str, destination: &PathBuf) -> Result<(), TextureError> {
    println!("Receiving slur: {}", slur);
    let url = "https://archive.org/download/pcsx2-hd-texture-packs";
    let html = fetch(url).await?;

    let document = Html::parse_document(&html);
    let selector = Selector::parse("table.directory-listing-table")?;
    let row = Selector::parse("tbody tr")?;
    let link = Selector::parse("a")?;

    if let Some(table) = document.select(&selector).next() {
        for row in table.select(&row) {
            if let Some(link) = row.select(&link).next() {
                // 1. check if the link contains the slur in the text only then check the href
                if let Some(href) = link.attr("href") {
                    let mut full_url = String::from(url);
                    full_url.push_str("/");
                    full_url.push_str(href);

                    let parsed_url = Url::parse(full_url.as_str())?;
                    println!("URL: {}", parsed_url);
                    // 2. then download the texture into the destination
                }
            }
        }
    }

    Ok(())
}