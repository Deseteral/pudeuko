use std::env;
use reqwest::{Client, ClientBuilder};
use reqwest::header::{HeaderMap, AUTHORIZATION};
use crate::domain::{ItemList};

fn make_client() -> Client {
    let dropbox_token = env::var("DROPBOX_TOKEN").expect("You have to provide a Dropbox API token");
    let authorization_header = format!("Bearer {}", dropbox_token);

    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, authorization_header.parse().unwrap());
    headers.insert("Dropbox-API-Arg", r#"{"path":"/pudeuko/data.json"}"#.parse().unwrap());

    let client = ClientBuilder::new().default_headers(headers).build().unwrap();
    client
}

pub fn fetch_pudeuko() -> ItemList {
    let body = make_client().get("https://content.dropboxapi.com/2/files/download")
        .send().unwrap()
        .text().unwrap();

   let items: ItemList = serde_json::from_str(&body).unwrap();
   items
}

pub fn upload_pudeuko(list: &ItemList) {

}
