#![feature(proc_macro_hygiene, decl_macro)]

mod config;
mod pudeuko;
mod api;

use rocket::{self, routes};
use pudeuko::dropbox_storage::DropboxStorage;
use pudeuko::pudeuko_service::PudeukoService;

fn main() {
    let config = config::Config::load();
    let dropbox_storage = DropboxStorage::new(&config.dropbox_token);
    let pudeuko_service = PudeukoService::new(Box::new(dropbox_storage));

    let mut config = rocket::Config::active().expect("Could not load configuration");
    config.set_port(config.port);

    rocket::custom(config)
        .manage(pudeuko_service)
        .mount("/items", routes![
            api::handlers::get_items,
            api::handlers::post_item,
            api::handlers::get_item,
        ])
        .launch();
}
