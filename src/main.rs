use std::path::PathBuf;

use crate::config::Config;
use crate::textures::TextureManager;

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
    listener
        .run(|path| {
            let path = path.to_string_lossy().to_string();

            let texture_manager = TextureManager::new(PathBuf::from(texture_destination.clone()));
            match iso_extractor::extract(&path) {
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
