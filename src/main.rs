mod api;
mod config;
mod domain;
mod dto;
mod infrastructure;
mod pudeuko_service;

use actix_web::{web, App, HttpServer};
use infrastructure::DropboxStorage;
use pudeuko_service::{PudeukoService, SharedPudeukoService};
use std::sync::{Arc, RwLock};

fn main() -> std::io::Result<()> {
    let app_config = config::Config::load();
    let dropbox_storage = DropboxStorage::new(&app_config.dropbox_token);
    let pudeuko_service = PudeukoService::new(Box::new(dropbox_storage));
    let shared_service: SharedPudeukoService = Arc::new(RwLock::new(pudeuko_service));

    HttpServer::new(move || {
        App::new()
            .data(shared_service.clone())
            .route("/items", web::get().to(api::items_endpoint::get_items))
            .route("/items", web::post().to(api::items_endpoint::post_item))
            .route("/items/{id}", web::get().to(api::items_endpoint::get_item))
    })
    .bind(format!("0.0.0.0:{}", &app_config.port))?
    .run()
}
