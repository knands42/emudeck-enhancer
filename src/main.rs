use std::path::PathBuf;

use crate::config::Config;
use crate::textures::TextureManager;

mod config;
mod extractor;
mod listener;
mod textures;

#[derive(Debug)]
enum AppError {
    Extractor(extractor::ExtractorError),
    TextureDownloaderError(textures::TextureDownloaderError),
    ListenerError(listener::ListenerError),
}

impl From<extractor::ExtractorError> for AppError {
    fn from(e: extractor::ExtractorError) -> Self {
        AppError::Extractor(e)
    }
}

impl From<textures::TextureDownloaderError> for AppError {
    fn from(e: textures::TextureDownloaderError) -> Self {
        AppError::TextureDownloaderError(e)
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
    let texture_manager = TextureManager::new(PathBuf::from(texture_destination));

    let listener = listener::Listener::new(root_path, &["iso"])?;
    listener
        .run(|path| {
            let path = path.to_string_lossy().to_string();

            match extractor::extract(&path) {
                Ok(game_info) => {
                    println!("path: {}", game_info.path.display());
                    println!("name: {}", game_info.name);
                    println!("serial: {:?}", game_info.serial);

                    if let Some(serial) = game_info.serial {
                        tokio::spawn(async move {
                            let _ = texture_manager.get_texture(&serial).await;
                        });
                    }
                }
                Err(e) => eprintln!("extract error for {}: {:?}", path, e),
            }
        })
        .await?;

    Ok(())
}
