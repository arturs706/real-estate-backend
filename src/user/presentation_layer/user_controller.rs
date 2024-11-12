use actix_web::web;

use crate::user::application_layer::user_service;

pub fn user_configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/users")
            .route("", web::get().to(user_service::get_all_users))
            .route("/byuserid/{user_id}", web::get().to(user_service::get_user_by_id))
            .route("", web::post().to(user_service::register_user))
            .route("", web::put().to(user_service::update_user))
            .route("/{user_id}", web::delete().to(user_service::delete_user))
            .route("/login", web::post().to(user_service::login_user))
            .route("/refresh", web::post().to(user_service::refresh_token))
            .route("/logout", web::post().to(user_service::logout_user))
            .route("/staff", web::get().to(user_service::get_user_full_names))
    );
}
