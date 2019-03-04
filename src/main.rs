#![feature(proc_macro_hygiene, decl_macro)]

mod config;
mod pudeuko;
mod api;

use rocket::{self, Rocket, routes};
use pudeuko::pudeuko_service::PudeukoService;

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
    let pudeuko_service = PudeukoService::new(&config.dropbox_token);

    rocket(config.port, pudeuko_service).launch();
}
