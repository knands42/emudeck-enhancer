use std::path::PathBuf;

use crate::config::Config;
use crate::usecase::TextureSyncFacade;

mod config;
mod iso_extractor;
mod listener;
mod textures;
mod usecase;

#[derive(Debug)]
enum AppError {
    Extractor(iso_extractor::ExtractorError),
    TextureError(textures::TextureError),
    ListenerError(listener::ListenerError),
}

impl From<iso_extractor::ExtractorError> for AppError {
    fn from(e: iso_extractor::ExtractorError) -> Self {
        AppError::Extractor(e)
    }
}

impl From<textures::TextureError> for AppError {
    fn from(e: textures::TextureError) -> Self {
        AppError::TextureError(e)
    }
}

impl From<listener::ListenerError> for AppError {
    fn from(e: listener::ListenerError) -> Self {
        AppError::ListenerError(e)
    }
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let config = Config::new();
    let root_path = config.root_path_to_listen;
    let texture_destination = config.texture_destination_path;

    let listener = listener::Listener::new(root_path, &["iso"])?;
    let sync = TextureSyncFacade::new(PathBuf::from(texture_destination));
    listener
        .run(|path| {
            let sync = sync.clone();
            let p = path.to_owned();
            tokio::spawn(async move {
                if let Err(e) = sync.process_iso(&p).await {
                    eprintln!("sync error: {e:?}");
                }
            });
        })
        .await?;

    Ok(())
}
