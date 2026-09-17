use std::env;

pub struct Config {
    pub root_path_to_listen: String,
    pub texture_destination_path: String
}

impl Config {
    pub fn new() -> Self {
        dotenvy::dotenv().ok();

        Config {
            root_path_to_listen: env::var("ROOT_PATH").unwrap_or_else(|_| ".".to_string()),
            texture_destination_path: env::var("TEXTURE_DESTINATION_PATH").unwrap_or_else(|_| ".".to_string())
        }
    }
}