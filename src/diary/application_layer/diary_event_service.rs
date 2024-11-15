use crate::diary::domain_layer::diary_event_types::{
    CreateEventRequest, DateQueryParams, Event, EventDetails,
};
use crate::diary::infrastructure_layer::diary_event_repo::EventRepository;
use crate::AppState;
use actix_web::{web, HttpResponse, Responder};
use chrono::NaiveDate;
use uuid::Uuid;

pub async fn get_all_events(state: web::Data<AppState>) -> impl Responder {
    let repo = EventRepository::new();
    match repo.get_all_events(state.into_inner()).await {
        Ok(events) => HttpResponse::Ok().json(events),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

pub async fn get_event_by_id(
    state: web::Data<AppState>,
    event_id: web::Path<Uuid>,
) -> impl Responder {
    let repo: EventRepository = EventRepository::new();
    match repo
        .get_event_by_id(state.into_inner(), event_id.into_inner())
        .await
    {
        Ok(event) => HttpResponse::Ok().json(event),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

pub async fn get_event_by_user_id(
    state: web::Data<AppState>,
    user_id: web::Path<Uuid>,
) -> impl Responder {
    let repo = EventRepository::new();

    match repo
        .get_user_events(state.into_inner(), user_id.into_inner())
        .await
    {
        Ok(events) => HttpResponse::Ok().json(events),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

pub async fn get_event_by_user_id_with_dates(
    state: web::Data<AppState>,
    user_id: web::Path<Uuid>,
    query: web::Query<DateQueryParams>,
) -> impl Responder {
    let repo = EventRepository::new();
    match repo
        .get_user_events_with_dates(
            state.into_inner(),
            user_id.into_inner(),
            query.start_date,
            query.end_date,
        )
        .await
    {
        Ok(events) => HttpResponse::Ok().json(events),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

pub async fn create_event(
    state: web::Data<AppState>,
    new_event_request: web::Json<CreateEventRequest>,
) -> impl Responder {
    let repo = EventRepository::new();
    let request = new_event_request.into_inner();
    match repo
        .create_event(state.into_inner(), request.event, request.details)
        .await
    {
        Ok(event) => HttpResponse::Created().json(event),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

pub async fn update_event(
    state: web::Data<AppState>,
    event_id: web::Path<Uuid>,
    updated_event: web::Json<Event>,
    updated_details: web::Json<EventDetails>,
) -> impl Responder {
    let repo = EventRepository::new();
    match repo
        .update_event(
            state.into_inner(),
            event_id.into_inner(),
            updated_event.into_inner(),
            updated_details.into_inner(),
        )
        .await
    {
        Ok(event) => HttpResponse::Ok().json(event),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

pub async fn delete_event(state: web::Data<AppState>, event_id: web::Path<Uuid>) -> impl Responder {
    let repo = EventRepository::new();
    match repo
        .delete_event(state.into_inner(), event_id.into_inner())
        .await
    {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

pub async fn get_events_by_date_range(
    state: web::Data<AppState>,
    start_date: web::Path<NaiveDate>,
    end_date: web::Path<NaiveDate>,
) -> impl Responder {
    let repo = EventRepository::new();
    match repo
        .get_events_by_date_range(
            state.into_inner(),
            start_date.into_inner(),
            end_date.into_inner(),
        )
        .await
    {
        Ok(events) => HttpResponse::Ok().json(events),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

pub async fn get_events_by_type(
    state: web::Data<AppState>,
    event_type: web::Path<String>,
) -> impl Responder {
    let repo = EventRepository::new();
    match repo
        .get_events_by_type(state.into_inner(), event_type.into_inner())
        .await
    {
        Ok(events) => HttpResponse::Ok().json(events),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}
