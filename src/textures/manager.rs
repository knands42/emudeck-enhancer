use std::path::PathBuf;


use crate::textures::downloader::download_texture;

pub struct TextureManager {
    texture_path: PathBuf,
    temp_dir: PathBuf
}

impl TextureManager {
    pub fn new(texture_path: PathBuf) -> Self {
        let new_tmp_dir = PathBuf::from("");
        create_dir(&new_tmp_dir);
        
        Self { texture_path, temp_dir: new_tmp_dir }
    }

    pub async fn get_texture(&self, slur: &str) {
        download_texture(slur, &self.temp_dir).await;
        self.extract_compact_file(slur, &self.texture_path);
    }
}

impl TextureManager {
    fn extract_compact_file(&self, slur: &str, destination_path: &PathBuf) {}
}

fn create_dir(destination_path: &PathBuf) {}
