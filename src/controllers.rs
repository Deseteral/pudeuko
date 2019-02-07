use rocket::{self, get, post};
use rocket_contrib::json::{Json};
use crate::domain::{ItemList, ContentDTO};
use crate::dropbox_client;
use crate::service;

#[get("/")]
pub fn get_items() -> String {
    let list: ItemList = dropbox_client::fetch_pudeuko();
    let json = serde_json::to_string(&list).unwrap();
    json
}

#[post("/", format = "application/json", data = "<content>")]
pub fn post_item(content: Json<ContentDTO>) -> String {
    let item = service::convert_content_to_item(content.0);
    let item_json = serde_json::to_string(&item).unwrap();

    let list = dropbox_client::fetch_pudeuko();
    let next_list = service::add_item_to_list(item, list);

    dropbox_client::upload_pudeuko(next_list);

    item_json
}
