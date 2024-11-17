use crate::properties::{
    domain_layer::property_core::PropertyCore,
    infrastructure_layer::properties_repository::PropertyRepository,
};
use crate::AppState;
use actix_web::{web, HttpResponse, Responder};

pub async fn get_all(state: web::Data<AppState>) -> impl Responder {
    let repo = PropertyRepository::new();
    match repo.get_all(state.into_inner()).await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

pub async fn add(state: web::Data<AppState>, property: web::Json<PropertyCore>) -> impl Responder {
    let repo = PropertyRepository::new();
    match repo.save_property(state.into_inner(), property.into_inner()).await {
        Ok(property) => HttpResponse::Ok().json(property),
        Err(e) => HttpResponse::InternalServerError().json(e),
    }
}
