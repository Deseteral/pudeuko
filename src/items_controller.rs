use rocket::{self, get, post};
use rocket_contrib::json::{Json};
use crate::domain::{ItemList, Item, ContentDTO};
use crate::dropbox_client;
use crate::service;

#[get("/")]
pub fn get_items() -> Json<ItemList> {
    let list: ItemList = dropbox_client::fetch_pudeuko();
    Json(list)
}

#[post("/", format = "application/json", data = "<content>")]
pub fn post_item(content: Json<ContentDTO>) -> Json<Item> {
    let item = service::convert_content_to_item(&content.0);
    let mut list = dropbox_client::fetch_pudeuko();

    service::add_item_to_list(item.clone(), &mut list);
    dropbox_client::upload_pudeuko(&list);

    Json(item)
}

#[get("/<id>")]
pub fn get_item(id: String) -> Option<Json<Item>> {
    let list = dropbox_client::fetch_pudeuko();
    let item = service::find_item_by_id(id, &list);

    item.map(|item| Json(item))
}
