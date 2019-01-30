#![feature(proc_macro_hygiene, decl_macro)]

use rocket::{self, get, routes, Config};
use std::env;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn configure() -> Config {
    let mut config = Config::active().expect("could not load configuration");
    if let Ok(port_str) = env::var("PORT") {
        let port = port_str.parse().expect("could not parse PORT");
        config.set_port(port);
    }
    config
}

fn main() {
    rocket::custom(configure()).mount("/", routes![index]).launch();
}
