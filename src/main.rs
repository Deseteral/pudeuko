mod api;
mod config;
mod logger;
mod domain;
mod dto;
mod infrastructure;
mod pudeuko_service;

use actix_web::{web, App, HttpServer};
use config::{Config, StorageType};
use infrastructure::{DropboxStorage, InMemoryStorage, Storage};
use log::{info};
use pudeuko_service::{PudeukoService, SharedPudeukoService};

fn main() -> std::io::Result<()> {
    logger::setup_logging().expect("Failed to initializer logger");

    let config = Config::load();
    let storage: Box<dyn Storage> = match config.storage_type {
        StorageType::Dropbox => Box::new(DropboxStorage::new(&config.dropbox_token)),
        StorageType::InMemory => Box::new(InMemoryStorage::new()),
    };
    let service = PudeukoService::new(storage);
    let shared_service = PudeukoService::make_shared(service);

    info!("Storage type: {}", &config.storage_type);

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
