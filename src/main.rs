#![feature(proc_macro_hygiene, decl_macro)]

use std::env;
use rocket::{self, routes, Config};

mod domain;
mod items_controller;
mod pudeuko_service;
mod dropbox_client;

fn create_configuration() -> Config {
    let mut config = Config::active().expect("Could not load configuration");
    if let Ok(port_str) = env::var("PORT") {
        let port = port_str.parse().expect("Could not parse PORT");
        config.set_port(port);
    }
    config
}

fn main() {
    let dropbox_token = env::var("DROPBOX_TOKEN").expect("You have to provide a Dropbox API token");
    let dropbox_client = dropbox_client::DropboxClient::new(dropbox_token);
    let pudeuko_service = pudeuko_service::PudeukoService::new(dropbox_client);

    let config = create_configuration();
    rocket::custom(config)
        .manage(pudeuko_service)
        .mount("/items", routes![
            items_controller::get_items,
            items_controller::post_item,
            items_controller::get_item,
        ])
        .launch();
}
