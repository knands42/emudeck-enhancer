use std::path::{Path, PathBuf};

use crate::textures::{TextureError, downloader::download_texture};

pub const TEMP_DIR_PREFIX: &str = "retro-station";

pub struct TextureManager {
    texture_path: PathBuf,
}

impl TextureManager {
    pub fn new(texture_path: PathBuf) -> Self {
        Self { texture_path }
    }

    pub async fn get_texture(&self, slur: &str) -> Result<(), TextureError> {
        let temp_dir = tempfile::Builder::new()
            .prefix(TEMP_DIR_PREFIX)
            .tempdir()?;

        download_texture(slur, temp_dir.path()).await?;
        create_dir(slur, &self.texture_path);
        extract_compact_file(slur, &self.texture_path);

        Ok(())
    }
}

fn extract_compact_file(slur: &str, destination_path: &Path) {
    
}

fn create_dir(slur: &str, destination_path: &Path) {
}
