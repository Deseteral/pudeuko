use std::env;

pub enum StorageType {
    InMemory,
    Dropbox,
}

pub struct Config {
    pub port: u16,
    pub dropbox_token: String,
    pub storage_type: StorageType,
}

impl Config {
    pub fn load() -> Self {
        Config::load_from_env()
    }

    fn load_from_env() -> Self {
        let port: u16 = env::var("PORT")
            .unwrap_or_else(|_| "8000".to_string())
            .parse()
            .expect("Could not parse PORT");

        let dropbox_token: String = env::var("DROPBOX_TOKEN")
            .unwrap_or_else(|_| "".to_string())
            .parse()
            .expect("Could not parse Dropbox API token");

        let storage_type = if dropbox_token.is_empty() {
            StorageType::InMemory
        } else {
            StorageType::Dropbox
        };

        Self {
            port,
            dropbox_token,
            storage_type,
        }
    }
}
