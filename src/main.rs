mod api;
mod config;
mod domain;
mod dto;
mod infrastructure;
mod pudeuko_service;

use actix_web::{web, App, HttpServer};
use config::{Config, StorageType};
use infrastructure::{DropboxStorage, InMemoryStorage, Storage};
use pudeuko_service::{PudeukoService, SharedPudeukoService};

fn main() -> std::io::Result<()> {
    let config = Config::load();
    let storage: Box<dyn Storage> = match config.storage_type {
        StorageType::Dropbox => Box::new(DropboxStorage::new(&config.dropbox_token)),
        StorageType::InMemory => Box::new(InMemoryStorage::new()),
    };
    let service = PudeukoService::new(storage);
    let shared_service = PudeukoService::make_shared(service);

    HttpServer::new(move || {
        App::new()
            .data(shared_service.clone())
            .route("/items", web::get().to(api::items_endpoint::get_items))
            .route("/items", web::post().to(api::items_endpoint::post_item))
            .route("/items/{id}", web::get().to(api::items_endpoint::get_item))
    })
    .bind(format!("127.0.0.1:{}", &config.port))?
    .run()
}
