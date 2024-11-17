#![allow(dead_code)]

use crate::{
    properties::domain_layer::property_core::{PropertyCore, PropertyStatus},
    AppState,
};
use actix_web::web::Json;
use serde_json::json;
use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;

pub struct PropertyRepository {}

impl PropertyRepository {
    pub fn new() -> Self {
        PropertyRepository {}
    }

    pub async fn save_property(&self, state: Arc<AppState>, property: PropertyCore) -> Result<PropertyCore, Json<String>> {
        let property_id = Uuid::new_v4();
        let query = r#"
            INSERT INTO property_core (property_id, status, property_type, letting_classification, staff_assigned, landlord_id, date_available, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            ON CONFLICT (property_id) 
            DO UPDATE 
            SET status = $2, property_type = $3, letting_classification = $4, staff_assigned = $5, landlord_id = $6, date_available = $7, updated_at = $9
            RETURNING *;
        "#;
        
        let result = sqlx::query_as::<_, PropertyCore>(query)
            .bind(&property_id)
            .bind(property.status)
            .bind(property.property_type)
            .bind(property.letting_classification)
            .bind(property.staff_assigned)
            .bind(property.landlord_id)
            .bind(property.date_available)
            .bind(property.created_at)
            .bind(property.updated_at)
            .fetch_one(&state.db)
            .await;
        
        match result {
            Ok(saved_property) => Ok(saved_property),
            Err(e) => Err(Json(e.to_string())),
        }
    }

    

    // Get all properties
    pub async fn get_all(&self, state: Arc<AppState>) -> Result<Vec<PropertyCore>, Json<String>> {
        let result = sqlx::query_as::<_, PropertyCore>("SELECT * FROM property_core")
            .fetch_all(&state.db)
            .await;
        match result {
            Ok(properties) => Ok(properties),
            Err(e) => Err(Json(e.to_string())),
        }
    }

    // Get one property by its ID
    pub async fn get_one_by_id(&self, property_id: Uuid, state: Arc<AppState>) -> Result<PropertyCore, Json<String>> {
        let result = sqlx::query_as::<_, PropertyCore>("SELECT * FROM property_core WHERE property_id = $1")
            .bind(property_id)
            .fetch_one(&state.db)
            .await;
        
        match result {
            Ok(property) => Ok(property),
            Err(e) => Err(Json(e.to_string())),
        }
    }

    // Get properties by landlord ID
    pub async fn get_one_by_user_id(&self, landlord_id: Uuid, state: Arc<AppState>) -> Result<Vec<PropertyCore>, Json<String>> {
        let result = sqlx::query_as::<_, PropertyCore>("SELECT * FROM property_core WHERE landlord_id = $1")
            .bind(landlord_id)
            .fetch_all(&state.db)
            .await;
        
        match result {
            Ok(properties) => Ok(properties),
            Err(e) => Err(Json(e.to_string())),
        }
    }
}
