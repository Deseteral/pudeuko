use rocket::{self, get, post};
use rocket_contrib::json::{Json};
use crate::domain::{ItemList, Item, ContentDTO};
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
    let item = service::convert_content_to_item(&content.0);
    let item_json = serde_json::to_string(&item).unwrap();

    let mut list = dropbox_client::fetch_pudeuko();
    service::add_item_to_list(item, &mut list);

    dropbox_client::upload_pudeuko(&list);

    item_json
}

#[get("/<id>")]
pub fn get_item(id: String) -> String {
    let list = dropbox_client::fetch_pudeuko();
    let item: &Item = service::find_item_by_id(id, &list).unwrap();
    let item_json = serde_json::to_string(item).unwrap();

    item_json
}
