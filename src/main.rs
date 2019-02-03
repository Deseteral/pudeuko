#![feature(proc_macro_hygiene, decl_macro)]

use rocket::{self, routes, Config};
use std::env;

mod controllers;
mod domain;
mod dropbox_client;

fn create_configuration() -> Config {
    let mut config = Config::active().expect("could not load configuration");
    if let Ok(port_str) = env::var("PORT") {
        let port = port_str.parse().expect("could not parse PORT");
        config.set_port(port);
    }
    config
}

fn main() {
    let config = create_configuration();
    rocket::custom(config)
        .mount("/items", routes![controllers::get_items, controllers::post_items])
        .launch();
}
