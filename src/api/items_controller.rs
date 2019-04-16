use crate::domain::Item;
use crate::dto::ContentDTO;
use crate::SharedPudeukoService;
use actix_web::{web, HttpResponse};

pub fn get_items(shared_service: web::Data<SharedPudeukoService>) -> HttpResponse {
    let service = shared_service.read().unwrap();
    let list = service.get_all();
    HttpResponse::Ok().json(list)
}

pub fn post_item(
    content: web::Json<ContentDTO>,
    shared_service: web::Data<SharedPudeukoService>,
) -> HttpResponse {
    let service = shared_service.read().unwrap();
    let item = Item::from(content.0);
    service.add_item(item.clone());
    HttpResponse::Ok().json(item)
}

pub fn get_item(
    path: web::Path<(String,)>,
    shared_service: web::Data<SharedPudeukoService>,
) -> HttpResponse {
    let service = shared_service.read().unwrap();
    let id = &path.0;

    match service.get_item_by_id(&id) {
        Some(item) => HttpResponse::Ok().json(item),
        None => HttpResponse::NotFound().body(format!("Item with id '{}' was not found", &id)),
    }
}

pub fn delete_item(
    path: web::Path<(String,)>,
    shared_service: web::Data<SharedPudeukoService>,
) -> HttpResponse {
    let service = shared_service.read().unwrap();
    let id = &path.0;

    if service.get_item_by_id(&id).is_some() {
        service.remove_item_by_id(&id);
        HttpResponse::NoContent().body(format!("Item with id '{}' was removed", &id))
    } else {
        HttpResponse::NotFound().body(format!("Item with id '{}' was not found", &id))
    }
}
