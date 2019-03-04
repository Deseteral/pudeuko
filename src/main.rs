#![feature(proc_macro_hygiene, decl_macro)]

mod config;
mod api;
mod domain;
mod pudeuko_service;
mod dropbox_client;

use rocket::{self, Rocket, routes};
use pudeuko_service::PudeukoService;

fn rocket(port: u16, pudeuko_service: PudeukoService) -> Rocket {
    let mut config = rocket::Config::active().expect("Could not load configuration");
    config.set_port(port);

    rocket::custom(config)
        .manage(pudeuko_service)
        .mount("/items", routes![
            api::handlers::get_items,
            api::handlers::post_item,
            api::handlers::get_item,
        ])
}

fn main() {
    let config = config::Config::load_from_env();
    let dropbox_client = dropbox_client::DropboxClient::new(&config.dropbox_token);
    let pudeuko_service = pudeuko_service::PudeukoService::new(dropbox_client);

    rocket(config.port, pudeuko_service).launch();
}
