use super::domain::ContentDTO;
use crate::pudeuko::domain::{Item, ItemList};
use crate::pudeuko::pudeuko_service::PudeukoService;
use rocket::{self, get, post, State};
use rocket_contrib::json::Json;

#[get("/")]
pub fn get_items(service: State<PudeukoService>) -> Json<ItemList> {
    let list = service.get_all();
    Json(list)
}

#[post("/", format = "application/json", data = "<content>")]
pub fn post_item(content: Json<ContentDTO>, service: State<PudeukoService>) -> Json<Item> {
    let item = Item::from(content.0);
    service.add_item(item.clone());
    Json(item)
}

#[get("/<id>")]
pub fn get_item(id: String, service: State<PudeukoService>) -> Option<Json<Item>> {
    service.get_item_by_id(id).map(|item| Json(item))
}
