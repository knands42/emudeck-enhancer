mod texture_sync_facade;

use crate::{iso_extractor, textures};
pub use texture_sync_facade::TextureSync;

#[derive(Debug)]
pub enum UseCaseError {
    Extract(iso_extractor::ExtractorError),
    Texture(textures::TextureError),
}

impl From<iso_extractor::ExtractorError> for UseCaseError {
    fn from(e: iso_extractor::ExtractorError) -> Self {
        Self::Extract(e)
    }
}
impl From<textures::TextureError> for UseCaseError {
    fn from(e: textures::TextureError) -> Self {
        Self::Texture(e)
    }
}
