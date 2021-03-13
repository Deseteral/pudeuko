use crate::domain::Item;
use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE};
use reqwest::{Client, ClientBuilder};
use serde_json::json;

const DROPBOX_FILE_PATH: &str = "/pudeuko/data.json";
const DROPBOX_DOWNLOAD_URL: &str = "https://content.dropboxapi.com/2/files/download";
const DROPBOX_UPLOAD_URL: &str = "https://content.dropboxapi.com/2/files/upload";

pub struct DropboxStorage {
    client: Client,
    download_headers: HeaderMap,
    upload_headers: HeaderMap,
}

impl DropboxStorage {
    pub fn new(dropbox_token: &str) -> Self {
        let mut default_headers = HeaderMap::new();
        default_headers.insert(
            AUTHORIZATION,
            format!("Bearer {}", &dropbox_token).parse().unwrap(),
        );

        let mut download_headers = HeaderMap::new();
        download_headers.insert(
            "Dropbox-API-Arg",
            json!({ "path": DROPBOX_FILE_PATH })
                .to_string()
                .parse()
                .unwrap(),
        );

        let mut upload_headers = HeaderMap::new();
        upload_headers.insert(CONTENT_TYPE, "application/octet-stream".parse().unwrap());
        upload_headers.insert(
            "Dropbox-API-Arg",
            json!({ "path": DROPBOX_FILE_PATH, "mode": "overwrite" })
                .to_string()
                .parse()
                .unwrap(),
        );

        Self {
            client: ClientBuilder::new()
                .default_headers(default_headers)
                .build()
                .unwrap(),
            download_headers,
            upload_headers,
        }
    }

    pub fn read(&self) -> Vec<Item> {
        let body = self
            .client
            .get(DROPBOX_DOWNLOAD_URL)
            .headers(self.download_headers.clone())
            .send()
            .unwrap()
            .text()
            .unwrap();

        let items: Vec<Item> = serde_json::from_str(&body).unwrap();
        items
    }

    pub fn write(&mut self, list: Vec<Item>) {
        let json = serde_json::to_string(&list).unwrap();

        self.client
            .post(DROPBOX_UPLOAD_URL)
            .headers(self.upload_headers.clone())
            .body(json)
            .send()
            .unwrap();
    }
}
