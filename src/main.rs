#![feature(proc_macro_hygiene, decl_macro)]

mod config;
mod domain;
mod items_controller;
mod pudeuko_service;
mod dropbox_client;

use rocket::{self, routes, Config};

fn create_configuration(port: &u16) -> Config {
    let mut config = Config::active().expect("Could not load configuration");
    config.set_port(*port);
    config
}

fn main() {
    let config = config::Config::load_from_env();
    let dropbox_client = dropbox_client::DropboxClient::new(&config.dropbox_token);
    let pudeuko_service = pudeuko_service::PudeukoService::new(dropbox_client);

    let config = create_configuration(&config.port);
    rocket::custom(config)
        .manage(pudeuko_service)
        .mount("/items", routes![
            items_controller::get_items,
            items_controller::post_item,
            items_controller::get_item,
        ])
        .launch();
}
