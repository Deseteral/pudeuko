use serde_derive::{Deserialize, Serialize};

pub type ItemList = Vec<Item>;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
  pub created_at: String,
  pub link: Link,
  pub text: String,
}

#[derive(Serialize, Deserialize)]
pub struct Link {
  pub url: String,
}
