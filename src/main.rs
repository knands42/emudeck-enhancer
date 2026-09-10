use reqwest::Error;
use scraper::{Html, Selector};

async fn fetch(url: &str) -> Result<String, Error> {
    reqwest::get(url).await?.text().await
}

#[tokio::main]
async fn main() {
    let url = "https://archive.org/download/pcsx2-hd-texture-packs";
    let html = fetch(url).await.expect("failed to fetch texture page");

    let document = Html::parse_document(&html);
    let selector = Selector::parse("table.directory-listing-table").unwrap();
    let row = Selector::parse("tbody tr").unwrap();
    let cell = Selector::parse("td").unwrap();
    let link = Selector::parse("a").unwrap();

    if let Some(table) = document.select(&selector).next() {
        for row in table.select(&row) {
            let cells: Vec<String> = row
                .select(&cell)
                .map(|td| td.inner_html().trim().to_string())
                .collect();

            if let Some(link) = row.select(&link).next() {
                let name = link.text().collect::<Vec<_>>().join(" ");

                println!("{:<60} {:<15} {} ",
                    name,
                    cells.get(1).unwrap_or(&"".to_string()),
                    cells.get(2).unwrap_or(&"".to_string())
                );
            }
        }
    }
}
