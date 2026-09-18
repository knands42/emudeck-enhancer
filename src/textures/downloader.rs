use std::path::Path;

use scraper::{Html, Selector};
use tokio::io::AsyncWriteExt;
use url::Url;

use crate::textures::TextureError;

pub async fn download_texture(slur: &str, destination: &Path) -> Result<(), TextureError> {
    println!("Receiving slur: {}", slur);
    let url = "https://archive.org/download/pcsx2-hd-texture-packs";
    let html = fetch(url).await?;
    let matched_url = get_downloadable_url(&url, slur, &html)?;

    if let Some(url) = matched_url {
        download_file(url, destination).await?;
    }

    Ok(())
}

async fn fetch(url: &str) -> Result<String, TextureError> {
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    Ok(body)
}

fn get_downloadable_url(url: &str, slur: &str, html: &str) -> Result<Option<Url>, TextureError> {
    let document = Html::parse_document(&html);
    let table_sel = Selector::parse("table.directory-listing-table")?;
    let row_sel = Selector::parse("tbody tr")?;
    let link_sel = Selector::parse("a")?;

    let slur_upper = slur.to_ascii_uppercase();
    let mut matched_url = None;
    for table in document.select(&table_sel) {
        for row in table.select(&row_sel) {
            let Some(link) = row.select(&link_sel).next() else {
                continue;
            };

            let name: String = link.text().collect();
            if !name.to_ascii_uppercase().contains(&slur_upper) {
                continue;
            }

            if let Some(href) = link.attr("href") {
                matched_url = Some(Url::parse(&format!("{url}/{href}"))?);
                break;
            }
        }
    }

    Ok(matched_url)
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
