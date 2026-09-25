use std::path::{Path, PathBuf};

use crate::textures::{
    TextureError, downloader::download_texture, extractor::extract_compact_file,
};

pub const TEMP_DIR_PREFIX: &str = "retro-station";

pub struct TextureManager {
    texture_path: PathBuf,
}

impl TextureManager {
    pub fn new(texture_path: PathBuf) -> Self {
        Self { texture_path }
    }

    pub async fn get_texture(&self, slur: &str) -> Result<(), TextureError> {
        let temp_dir = tempfile::Builder::new().prefix(TEMP_DIR_PREFIX).tempdir()?;

        println!("tmp_dir: {}", format!("{:?}", temp_dir.path().clone()));

        download_texture(slur, temp_dir.path()).await?;
        let destination_path = create_dir(slur, &self.texture_path).await?;
        extract_compact_file(temp_dir.path(), &destination_path);

        Ok(())
    }
}

async fn create_dir(slur: &str, destination_path: &Path) -> Result<PathBuf, TextureError> {
    let game_dir = destination_path.join(slur);
    tokio::fs::create_dir_all(&game_dir).await?;
    Ok(game_dir)
}
