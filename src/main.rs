mod api;
mod config;
mod domain;
mod dto;
mod infrastructure;
mod pudeuko_service;

use actix_web::{web, App, HttpServer};
use config::Config;
use infrastructure::DropboxStorage;
use pudeuko_service::{PudeukoService, SharedPudeukoService};

fn main() -> std::io::Result<()> {
    let config = Config::load();
    let storage = DropboxStorage::new(&config.dropbox_token);
    let service = PudeukoService::new(Box::new(storage));
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
