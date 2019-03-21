use std::env;

pub struct Config {
    pub port: u16,
    pub dropbox_token: String,
}

impl Config {
    pub fn load() -> Self {
        Config::load_from_env()
    }

    fn load_from_env() -> Self {
        let port = env::var("PORT")
            .unwrap_or_else(|_| "8000".to_string())
            .parse()
            .expect("Could not parse PORT");

        let dropbox_token =
            env::var("DROPBOX_TOKEN").expect("You have to provide a Dropbox API token");

        Self {
            port,
            dropbox_token,
        }
    }
}
