use std::env;

pub struct Config {
    pub root_path: String,
}

impl Config {
    pub fn new() -> Self {
        dotenvy::dotenv().ok();

        Config {
            root_path: env::var("ROOT_PATH").unwrap_or_else(|_| ".".to_string()),
        }
    }
}