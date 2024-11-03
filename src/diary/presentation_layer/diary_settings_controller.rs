use actix_web::web;

use crate::diary::application_layer::diary_settings_service;

pub fn diary_settings_configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/diary-settings")
            .route(
                "",
                web::get().to(diary_settings_service::get_all_diary_settings),
            ) // Get all diary settings
            .route(
                "/{staff_id}",
                web::get().to(diary_settings_service::get_diary_settings_by_id),
            ) // Get a specific diary setting by id
            .route(
                "",
                web::post().to(diary_settings_service::create_diary_settings),
            ) // Create new diary settings
            .route(
                "/{diary_id}",
                web::put().to(diary_settings_service::update_diary_settings),
            ) // Update a specific diary setting
            .route(
                "/{diary_id}",
                web::delete().to(diary_settings_service::delete_diary_settings),
            ), // Delete a specific diary setting
    );
}
