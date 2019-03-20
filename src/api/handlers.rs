use rocket::{self, get, post, State};
use rocket_contrib::json::{Json};
use super::domain::{ContentDTO};
use crate::pudeuko::domain::{ItemList, Item};
use crate::api::domain::SharedPudeukoService;

#[get("/")]
pub fn get_items(shared_service: State<SharedPudeukoService>) -> Json<ItemList> {
    let list = shared_service
        .clone().read().unwrap()
        .get_all();

    Json(list)
}

#[post("/", format = "application/json", data = "<content>")]
pub fn post_item(content: Json<ContentDTO>, shared_service: State<SharedPudeukoService>) -> Json<Item> {
    let item = Item::from(content.0);

    shared_service
        .clone().read().unwrap()
        .add_item(item.clone());

    Json(item)
}

#[get("/<id>")]
pub fn get_item(id: String, shared_service: State<SharedPudeukoService>) -> Option<Json<Item>> {
    shared_service
        .clone().read().unwrap()
        .get_item_by_id(id)
        .map(|item| Json(item))
}
