use crate::landlord::application_layer::landlord_service;
use actix_web::web;

pub fn landlord_configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/landlords")
            .route("", web::get().to(landlord_service::get_all_landlords)) 
            .route("", web::post().to(landlord_service::register_landlord))
    );
}
