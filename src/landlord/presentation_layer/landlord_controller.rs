use actix_web::web;
use crate::landlord::application_layer::landlord_service;

// presentation_layer/landlord_controller.rs

pub fn landlord_configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/landlords")
            .route("", web::get().to(landlord_service::get_all_landlords))
            // Registration routes
            .route("/registration", web::post().to(landlord_service::start_registration))
            .route("/registration", web::get().to(landlord_service::get_user_registrations))
            .route("/registration/{registration_id}", 
                web::get().to(landlord_service::get_registration_details))
            .route("/registration/{registration_id}/basicinfo", 
                web::post().to(landlord_service::save_basic_info))
            .route("/registration/{registration_id}/general", 
                web::post().to(landlord_service::save_general_info))
            .route("/registration/{registration_id}/address", 
                web::post().to(landlord_service::save_address))
            .route("/registration/{registration_id}/bankdetails", 
                web::post().to(landlord_service::save_bank_details))
            .route("/registration/{registration_id}/lettingspreferences", 
                web::post().to(landlord_service::save_lettings_preferences))
            .route("/registration/{registration_id}/complete", 
                web::post().to(landlord_service::complete_registration))
    );
}