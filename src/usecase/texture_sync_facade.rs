use std::path::{Path, PathBuf};

use crate::{iso_extractor, textures::TextureManager, usecase::UseCaseError};

#[derive(Clone, Debug)]
pub struct TextureSyncFacade {
    tex_dest: std::path::PathBuf,
}

impl TextureSyncFacade {
    pub fn new(tex_dest: PathBuf) -> Self {
        Self { tex_dest: tex_dest }
    }
    pub async fn process_iso(&self, iso_path: &Path) -> Result<(), UseCaseError> {
        let game_info = iso_extractor::extract(&iso_path.to_string_lossy())?;
        if let Some(serial) = game_info.serial {
            let mgr = TextureManager::new(self.tex_dest.clone());
            mgr.get_texture(&serial).await?;
        }
        Ok(())
    }
}
