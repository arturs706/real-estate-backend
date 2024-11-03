use crate::diary::domain_layer::diary_settings::DiarySettings;
use crate::diary::infrastructure_layer::diary_settings_repo::DiarySettingsRepository;
use crate::AppState;
use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid;

pub async fn get_all_diary_settings(state: web::Data<AppState>) -> impl Responder {
    let repo = DiarySettingsRepository::new();
    match repo.get_all_diary_settings(state.into_inner()).await {
        Ok(settings) => HttpResponse::Ok().json(settings),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

pub async fn get_diary_settings_by_id(
    state: web::Data<AppState>,
    staff_id: web::Path<Uuid>,
) -> impl Responder {
    let repo = DiarySettingsRepository::new();
    match repo
        .get_diary_settings_by_id(state.into_inner(), staff_id.into_inner())
        .await
    {
        Ok(settings) => HttpResponse::Ok().json(settings),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

pub async fn create_diary_settings(
    state: web::Data<AppState>,
    new_settings: web::Json<DiarySettings>,
) -> impl Responder {
    let repo = DiarySettingsRepository::new();
    match repo
        .create_diary_settings(state.into_inner(), new_settings.into_inner())
        .await
    {
        Ok(settings) => HttpResponse::Created().json(settings),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

pub async fn update_diary_settings(
    state: web::Data<AppState>,
    diary_id: web::Path<Uuid>,
    updated_settings: web::Json<DiarySettings>,
) -> impl Responder {
    let repo = DiarySettingsRepository::new();
    match repo
        .update_diary_settings(
            state.into_inner(),
            diary_id.into_inner(),
            updated_settings.into_inner(),
        )
        .await
    {
        Ok(settings) => HttpResponse::Ok().json(settings),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

pub async fn delete_diary_settings(
    state: web::Data<AppState>,
    diary_id: web::Path<Uuid>,
) -> impl Responder {
    let repo = DiarySettingsRepository::new();
    match repo
        .delete_diary_settings(state.into_inner(), diary_id.into_inner())
        .await
    {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}
