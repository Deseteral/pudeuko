use std::{env, fmt};

pub enum StorageType {
    InMemory,
    Dropbox(String),
}

pub struct Config {
    pub port: u16,
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
            StorageType::Dropbox(dropbox_token)
        };

        Self {
            port,
            storage_type,
        }
    }
}

impl fmt::Display for StorageType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StorageType::Dropbox(_token) => write!(f, "Dropbox"),
            StorageType::InMemory => write!(f, "In memory"),
        }
    }
}
