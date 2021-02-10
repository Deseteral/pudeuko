use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: String,
    pub created_at: String,
    pub link: Option<Link>,
    pub text: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Link {
    pub url: String,
}
