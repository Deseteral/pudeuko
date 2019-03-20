use std::env;

pub struct Config {
    pub port: u16,
    pub dropbox_token: String,
}

impl Config {
    pub fn load_from_env() -> Self {
        let port = (match env::var("PORT") {
            Ok(port_str) => port_str.parse(),
            Err(_) => Ok(8000),
        }).expect("Could not parse PORT");

        let dropbox_token = env::var("DROPBOX_TOKEN")
            .expect("You have to provide a Dropbox API token");

        Self { port, dropbox_token }
    }
}
