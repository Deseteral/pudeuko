mod api;
mod config;
mod domain;
mod dto;
mod infrastructure;
mod logger;
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

    logger::setup().expect("Failed to initialize logger");

    HttpServer::new(move || {
        App::new()
            .data(shared_service.clone())
            .service(web::resource("/items")
                .route(web::get().to(api::items_controller::get_items))
                .route(web::post().to(api::items_controller::post_item))
            )
            .service(web::resource("/items/{id}")
                .route(web::delete().to(api::items_controller::delete_item))
                .route(web::get().to(api::items_controller::get_item))
            )
    })
    .bind(format!("0.0.0.0:{}", &app_config.port))?
    .run()
}
