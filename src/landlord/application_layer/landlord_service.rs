use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use uuid::Uuid;

use crate::{
    landlord::infrastructure_layer::landlord_repository::LandlordRepository,
    AppState,
    landlord::domain_layer::{
        landlord_details::BasicInfoRequest,
        landlord_general::LandlordGeneralRequest,
        landlords_address::AddressRequest,
        landlords_bank_details::BankDetailsRequest,
        landlords_lettings_management::LettingsPreferencesRequest,
        landlord_registration_progress::RegistrationStep,
    },
};

pub async fn get_user_registrations(
    state: web::Data<AppState>,
    user_id: web::Query<Uuid>,
) -> impl Responder {
    let repo = LandlordRepository::new();
    match repo.get_user_registrations(state.into_inner(), user_id.into_inner()).await {
        Ok(registrations) => HttpResponse::Ok().json(registrations),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

pub async fn get_registration_details(
    state: web::Data<AppState>,
    registration_id: web::Path<Uuid>,
) -> impl Responder {
    let repo = LandlordRepository::new();
    match repo.get_registration_details(state.into_inner(), registration_id.into_inner()).await {
        Ok(Some(details)) => HttpResponse::Ok().json(details),
        Ok(None) => HttpResponse::NotFound().json("Registration not found"),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}


pub async fn start_registration(
    state: web::Data<AppState>,
    user_id: Option<web::Query<Uuid>>,
) -> impl Responder {
    let repo = LandlordRepository::new();
    match repo.start_or_resume_registration(
        state.into_inner(),
        user_id.map(|id| id.into_inner()),
    ).await {
        Ok(progress) => HttpResponse::Ok().json(progress),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

pub async fn get_all_landlords(state: web::Data<AppState>) -> impl Responder {
    let repo = LandlordRepository::new();
    match repo.get_all(state.into_inner()).await {
        Ok(landlords) => HttpResponse::Ok().json(landlords),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}


pub async fn save_basic_info(
    state: web::Data<AppState>,
    registration_id: web::Path<Uuid>,
    basic_info: web::Json<BasicInfoRequest>,
) -> impl Responder {
    let repo = LandlordRepository::new();
    let step_data = json!({
        "basicinfo": basic_info.into_inner()
    });
    
    match repo.save_registration_step(
        state.into_inner(),
        registration_id.into_inner(),
        RegistrationStep::Contactdetails,
        step_data,
    ).await {
        Ok(_) => HttpResponse::Ok().json(json!({"message": "Basic info saved"})),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

pub async fn save_general_info(
    state: web::Data<AppState>,
    registration_id: web::Path<Uuid>,
    general_info: web::Json<LandlordGeneralRequest>,
) -> impl Responder {
    let repo = LandlordRepository::new();
    let step_data = json!({
        "general": general_info.into_inner()
    });
    
    match repo.save_registration_step(
        state.into_inner(),
        registration_id.into_inner(),
        RegistrationStep::Address,
        step_data,
    ).await {
        Ok(_) => HttpResponse::Ok().json(json!({"message": "General info saved"})),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

pub async fn save_address(
    state: web::Data<AppState>,
    registration_id: web::Path<Uuid>,
    address: web::Json<AddressRequest>,
) -> impl Responder {
    let repo = LandlordRepository::new();
    let step_data = json!({
        "address": address.into_inner()
    });
    
    match repo.save_registration_step(
        state.into_inner(),
        registration_id.into_inner(),
        RegistrationStep::Bankdetails,
        step_data,
    ).await {
        Ok(_) => HttpResponse::Ok().json(json!({"message": "Address saved"})),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

pub async fn save_bank_details(
    state: web::Data<AppState>,
    registration_id: web::Path<Uuid>,
    bank_details: web::Json<BankDetailsRequest>,
) -> impl Responder {
    let repo = LandlordRepository::new();
    let step_data = json!({
        "bankdetails": bank_details.into_inner()
    });
    
    match repo.save_registration_step(
        state.into_inner(),
        registration_id.into_inner(),
        RegistrationStep::Lettingspreferences,
        step_data,
    ).await {
        Ok(_) => HttpResponse::Ok().json(json!({"message": "Bank details saved"})),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

pub async fn save_lettings_preferences(
    state: web::Data<AppState>,
    registration_id: web::Path<Uuid>,
    preferences: web::Json<LettingsPreferencesRequest>,
) -> impl Responder {
    let repo = LandlordRepository::new();
    let step_data = json!({
        "lettingspreferences": preferences.into_inner()
    });
    
    match repo.save_registration_step(
        state.into_inner(),
        registration_id.into_inner(),
        RegistrationStep::Completed,
        step_data,
    ).await {
        Ok(_) => HttpResponse::Ok().json(json!({"message": "Lettings preferences saved"})),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

pub async fn complete_registration(
    state: web::Data<AppState>,
    registration_id: web::Path<Uuid>,
) -> impl Responder {
    let repo = LandlordRepository::new();
    match repo.complete_registration(state.into_inner(), registration_id.into_inner()).await {
        Ok(landlord_id) => HttpResponse::Ok().json(json!({
            "message": "Registration completed successfully",
            "landlord_id": landlord_id
        })),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}