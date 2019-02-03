use std::env;
use reqwest::{Client, ClientBuilder};
use reqwest::header::{HeaderMap, AUTHORIZATION};
use crate::domain::{ItemList};

fn make_client() -> Result<Client, reqwest::Error> {
    let dropbox_token: String = env::var("DROPBOX_TOKEN").expect("You have to provide a Dropbox API token");
    let authorization_header: String = format!("Bearer {}", dropbox_token);

    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, authorization_header.parse().unwrap());
    headers.insert("Dropbox-API-Arg", r#"{"path":"/pudeuko/data.json"}"#.parse().unwrap());

    let client = ClientBuilder::new().default_headers(headers).build()?;

    Ok(client)
}

pub fn fetch_pudeuko() -> Result<(), reqwest::Error> {
    let client = make_client().expect("Could not set up Dropbox HTTP client");
    let body = client.get("https://content.dropboxapi.com/2/files/download")
        .send()?
        .text()?;

    dbg!(body);
    Ok(())
}
