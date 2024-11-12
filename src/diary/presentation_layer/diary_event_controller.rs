use actix_web::web;

use crate::diary::application_layer::diary_event_service;

// PRESENTATION LAYER (routes.rs)
pub fn diary_event_configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/events")
            .route("", web::get().to(diary_event_service::get_all_events))
            .route("", web::post().to(diary_event_service::create_event))
            .route(
                "/{event_id}",
                web::get().to(diary_event_service::get_event_by_id),
            )
            .route(
                "/{event_id}",
                web::put().to(diary_event_service::update_event),
            )
            .route(
                "/{event_id}",
                web::delete().to(diary_event_service::delete_event),
            )
            .route(
                "/users/{user_id}",
                web::get().to(diary_event_service::get_event_by_user_id),
            )
            .route(
                "/diary/{user_id}/events",
                web::get().to(diary_event_service::get_event_by_user_id_with_dates),
            ),
    );
}
