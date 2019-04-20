mod api;
mod config;
mod domain;
mod dto;
mod infrastructure;
mod logger;
mod pudeuko_service;

use actix_web::{web, App, HttpServer};
use config::{Config, StorageType};
use infrastructure::{DropboxStorage, InMemoryStorage, Storage};
use log::info;
use pudeuko_service::{PudeukoService, SharedPudeukoService};

fn main() -> std::io::Result<()> {
    logger::setup().expect("Failed to initialize logger");

    let config = Config::load();
    info!("Storage type: {}", &config.storage_type);

    let storage: Box<dyn Storage> = match config.storage_type {
        StorageType::Dropbox(token) => Box::new(DropboxStorage::new(&token)),
        StorageType::InMemory => Box::new(InMemoryStorage::new()),
    };
    let service = PudeukoService::new(storage);
    let shared_service = PudeukoService::make_shared(service);

    HttpServer::new(move || {
        App::new()
            .data(shared_service.clone())
            .route("/items", web::get().to(api::items_controller::get_items))
            .route("/items", web::post().to(api::items_controller::post_item))
            .route(
                "/items/{id}",
                web::get().to(api::items_controller::get_item),
            )
    })
    .bind(format!("0.0.0.0:{}", &config.port))?
    .run()
}
