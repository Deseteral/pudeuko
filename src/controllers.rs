use rocket::{self, post};
use rocket_contrib::json::{Json};
use crate::domain::{ItemList};

#[post("/", format = "application/json", data = "<items>")]
pub fn post_items(items: Json<ItemList>) -> String {
  items.len().to_string()
}
