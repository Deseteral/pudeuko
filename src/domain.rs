use serde_derive::{Deserialize, Serialize};

pub type ItemList = Vec<Item>;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: String,
    pub created_at: String,
    pub link: Link,
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Link {
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContentDTO {
    pub text: String,
}
