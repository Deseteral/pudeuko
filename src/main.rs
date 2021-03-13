mod config;
mod domain;
mod dropbox_storage;
mod dto;
mod items_controller;
mod logger;
mod pudeuko_service;

use actix_web::{web, App, HttpServer};
use config::Config;
use dropbox_storage::DropboxStorage;
use pudeuko_service::{PudeukoService, SharedPudeukoService};

fn main() -> std::io::Result<()> {
    logger::setup().expect("Failed to initialize logger");

    let config = Config::load_from_env();

    let storage = DropboxStorage::new(&config.dropbox_token);
    let service = PudeukoService::new(storage);
    let shared_service = PudeukoService::make_shared(service);

    HttpServer::new(move || {
        App::new()
            .data(shared_service.clone())
            .service(
                web::resource("/items")
                    .route(web::get().to(items_controller::get_items))
                    .route(web::post().to(items_controller::post_item)),
            )
            .service(
                web::resource("/items/{id}")
                    .route(web::delete().to(items_controller::delete_item))
                    .route(web::get().to(items_controller::get_item)),
            )
    })
    .bind(format!("0.0.0.0:{}", &config.port))?
    .run()
}
