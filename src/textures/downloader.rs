use std::path::Path;

use scraper::{Html, Selector};
use tokio::io::AsyncWriteExt;
use url::Url;

use crate::textures::TextureError;

async fn fetch(url: &str) -> Result<String, TextureError> {
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    Ok(body)
}

async fn download_file(url: Url, destination: &Path) -> Result<(), TextureError> {
    let mut response = reqwest::get(url).await?.error_for_status()?;
    let mut file = tokio::fs::File::create(destination).await?;
    while let Some(chunk) = response.chunk().await? {
        file.write_all(&chunk).await?;
    }
    file.flush().await?;
    Ok(())
}

pub async fn download_texture(slur: &str, destination: &Path) -> Result<(), TextureError> {
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
                let name: String = link.text().collect();
                if !name
                    .to_ascii_uppercase()
                    .contains(&slur.to_ascii_uppercase())
                {
                    continue;
                }

                if let Some(href) = link.attr("href") {
                    let mut full_url = String::from(url);
                    full_url.push_str("/");
                    full_url.push_str(href);

                    let parsed_url = Url::parse(full_url.as_str())?;
                    println!("URL: {}", parsed_url);

                    download_file(parsed_url, destination);
                }
            }
        }
    }

    Ok(())
}
