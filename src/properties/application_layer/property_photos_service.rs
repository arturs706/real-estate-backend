use crate::properties::{
    domain_layer::property_images::PropertyImages,
    infrastructure_layer::property_images_repository::PropertyImagesRepository,
};
use crate::AppState;
use actix_multipart::Multipart;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use uuid::Uuid;


pub async fn add(
    state: web::Data<AppState>,
    property_photos: web::Json<PropertyImages>,
) -> impl Responder {
    let repo = PropertyImagesRepository::new();

    match repo
        .save(state.into_inner(), property_photos.into_inner())
        .await
    {
        Ok(property_photos) => HttpResponse::Ok().json(property_photos),
        Err(e) => HttpResponse::InternalServerError().json(e),
    }
}

pub async fn get_by_id(state: web::Data<AppState>, property_id: web::Path<Uuid>) -> impl Responder {
    let repo = PropertyImagesRepository::new();

    match repo
        .get_by_id(state.into_inner(), property_id.into_inner())
        .await
    {
        Ok(property_photos) => HttpResponse::Ok().json(property_photos),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}
pub async fn upload_images(
    state: web::Data<AppState>,
    property_id: web::Path<Uuid>,
    mut payload: Multipart,
    req: HttpRequest,
) -> impl Responder {
    let repo = PropertyImagesRepository::new();
    println!("upload_images");
    match repo
        .upload_images(
            state.into_inner(),
            property_id.into_inner(),
            &mut payload,
            req.into(),
        )
        .await
    {
        Ok((status_code, property_photos)) => {
            HttpResponse::build(status_code).json(property_photos)
        }
        Err((status_code, e)) => HttpResponse::build(status_code).json(e),
    }
}
