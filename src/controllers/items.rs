use rocket::{self, post};

#[post("/")]
pub fn items() -> &'static str {
  "Nice post"
}
