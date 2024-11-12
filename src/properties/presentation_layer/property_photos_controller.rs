use actix_web::web;

use crate::properties::application_layer::property_photos_service;

pub fn configure_photos_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api/v1/properties/photos").route(
        "/{property_id}",
        web::post().to(property_photos_service::upload_images),
    ));
}
