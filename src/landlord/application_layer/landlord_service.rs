use actix_web::{web, HttpResponse, Responder};

use crate::{landlord::{domain_layer::landlord_details::{LandlordDetails, LandlordQueryParams}, infrastructure_layer::landlord_repository::LandlordRepository}, AppState};

pub async fn get_all_landlords(
    state: web::Data<AppState>,
    query: web::Query<LandlordQueryParams>,
) -> impl Responder {
    let repo = LandlordRepository::new();
    match repo.get_all(state.into_inner(), query).await {
        Ok(landlords) => HttpResponse::Ok().json(landlords),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

pub async fn register_landlord(
    state: web::Data<AppState>,
    landlord: web::Json<LandlordDetails>,
) -> impl Responder {
    let repo = LandlordRepository::new();
    match repo.save_details(state.into_inner(), landlord.into_inner()).await {
        Ok(saved_landlord) => HttpResponse::Ok().json(saved_landlord),
        Err(e) => HttpResponse::InternalServerError().json(e),
    }
}


