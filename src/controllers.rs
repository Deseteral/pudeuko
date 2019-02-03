use rocket::{self, get, post};
use rocket_contrib::json::{Json};
use crate::domain::{ItemList, ContentDTO};
use crate::dropbox_client;

#[get("/")]
pub fn get_items() -> String {
    dropbox_client::fetch_pudeuko();
    "".to_string()
}

#[post("/", format = "application/json", data = "<items>")]
pub fn post_items(items: Json<ContentDTO>) -> String {
    "".to_string()
}
