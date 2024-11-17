#![allow(
    clippy::needless_borrow,
    clippy::needless_return,
    clippy::upper_case_acronyms,
    dead_code
)]
use actix_cors::Cors;
use actix_web::{http, web::Data, App, HttpServer};
use diary::presentation_layer::{
    diary_event_controller::diary_event_configure_routes,
    diary_settings_controller::diary_settings_configure_routes,
};
use dotenv::dotenv;
use landlord::presentation_layer::landlord_controller::landlord_configure_routes;
use listenfd::ListenFd;
use properties::presentation_layer::{
    properties_controller::configure_routes, property_address_controller::configure_address_routes,
    property_images_controller::configure_photos_routes,
};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::fs;
use user::presentation_layer::user_controller::user_configure_routes;
mod properties {
    pub mod application_layer;
    pub mod domain_layer;
    pub mod infrastructure_layer;
    pub mod presentation_layer;
}
mod diary {
    pub mod application_layer;
    pub mod domain_layer;
    pub mod infrastructure_layer;
    pub mod presentation_layer;
}

mod user {
    pub mod application_layer;
    pub mod domain_layer;
    pub mod infrastructure_layer;
    pub mod presentation_layer;
}
mod landlord {
    pub mod application_layer;
    pub mod domain_layer;
    pub mod infrastructure_layer;
    pub mod presentation_layer;
}

#[derive(Clone)]
pub struct AppState {
    pub db: Pool<Postgres>,
    pub jwt_secret: String,
    pub upload_dir: String,
}

fn initialize_upload_directory() -> std::io::Result<String> {
    let upload_dir =
        std::env::var("UPLOAD_DIR").unwrap_or_else(|_| "./uploads/id_documents".to_string());
    fs::create_dir_all(&upload_dir)?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let permissions = fs::Permissions::from_mode(0o750); // rwxr-x---
        fs::set_permissions(&upload_dir, permissions)?;
    }

    Ok(upload_dir)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL_RO").expect("DATABASE_URL_RO must be set");
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let server_port = std::env::var("SERVER_PORT").expect("SERVER_PORT must be set");
    let server_host = std::env::var("SERVER_HOST").expect("SERVER_HOST must be set");

    let server_ip = format!("{}:{}", server_host, server_port);
    let upload_dir = initialize_upload_directory().expect("Failed to initialize upload directory");

    let pool = PgPoolOptions::new()
        .max_connections(1000)
        .connect(&database_url)
        .await
        .expect("Failed to create pool");
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState {
                db: pool.clone(),
                jwt_secret: jwt_secret.clone(),
                upload_dir: upload_dir.clone(),
            }))
            .wrap(
                Cors::default() // Add CORS middleware here
                    .allowed_origin("http://localhost:3000") // Adjust the origin as necessary
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"]) // Specify allowed methods
                    .allowed_headers(vec![
                        http::header::AUTHORIZATION,
                        http::header::ACCEPT,
                        http::header::CONTENT_TYPE,
                    ]) // Specify allowed headers
                    .max_age(3600),
            ) // Optional: Cache the preflight response
            .configure(user_configure_routes)
            .configure(configure_photos_routes)
            .configure(configure_routes)
            .configure(configure_address_routes)
            .configure(diary_settings_configure_routes)
            .configure(diary_event_configure_routes)
            .configure(landlord_configure_routes)

        // .wrap(infrastructure_layer::auth_repo::Auth)
    });
    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => server.bind(server_ip)?,
    };
    // let topics = "images";
    // let brokers = &server_host.as_str();
    // let group_id = "0";

    // consume_and_print(brokers, group_id, topics).await;
    server.run().await.unwrap();
    Ok(())
}
