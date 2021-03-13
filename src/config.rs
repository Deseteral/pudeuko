use std::env;

pub struct Config {
    pub port: u16,
    pub dropbox_token: String,
}

impl Config {
    pub fn load_from_env() -> Self {
        let port: u16 = env::var("PORT")
            .unwrap_or_else(|_| "8000".to_string())
            .parse()
            .expect("Could not parse PORT");

        let dropbox_token: String =
            env::var("DROPBOX_TOKEN").expect("Could not parse Dropbox API token");

        Self {
            port,
            dropbox_token,
        }
    }
}
