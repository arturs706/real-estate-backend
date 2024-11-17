use actix_web::error::ResponseError;
use actix_web::web;
use chrono::Utc;
use derive_more::Display;
use serde::Serialize;
use serde_json::json;
use sqlx::types::JsonValue;
use std::sync::Arc;
use uuid::Uuid;

use crate::landlord::domain_layer::landlord_details::LandlordQueryParams;
use crate::{
    landlord::domain_layer::
    landlord_details::LandlordDetails,
    AppState,
};

#[derive(Debug, Display, Serialize)]
pub enum CustomErrors {
    #[display(fmt = "Database error: {}", _0)]
    DatabaseError(String),
    #[display(fmt = "Not authorized")]
    NotAuthorized,
    #[display(fmt = "Validation error: {}", _0)]
    ValidationError(String),
}

impl ResponseError for CustomErrors {}

pub struct LandlordRepository {}

impl LandlordRepository {
    pub fn new() -> Self {
        LandlordRepository {}
    }

    pub async fn get_all(
        &self,
        state: Arc<AppState>,
        query: web::Query<LandlordQueryParams>,
    ) -> Result<Vec<LandlordDetails>, CustomErrors> {
        let base_query = "SELECT * FROM landlord_details";
        let mut query_string = base_query.to_string();
        let mut conditions = Vec::new();
    
        if let Some(status) = &query.status {
            match status.to_lowercase().as_str() {
                "active" | "inactive" => {
                    conditions.push(format!("status = '{}'", status.to_lowercase()));
                }
                "all" => {} 
                _ => return Err(CustomErrors::ValidationError("Invalid status value".to_string())),
            }
        }
    
        if !conditions.is_empty() {
            query_string.push_str(" WHERE ");
            query_string.push_str(&conditions.join(" AND "));
        }
    
        if let Some(sort_by) = &query.sort_by {
            let order = query.order.as_deref().unwrap_or("ASC");
            
            if !["ASC", "DESC", "asc", "desc"].contains(&order) {
                return Err(CustomErrors::ValidationError("Invalid order direction".to_string()));
            }
            
            let order = order.to_uppercase();
            
            match sort_by.to_lowercase().as_str() {
                "name" => {
                    query_string.push_str(&format!(
                        " ORDER BY COALESCE(full_name, company_name) {}, COALESCE(company_name, full_name) {}",
                        order, order
                    ));
                }
                "created_at" | "updated_at" | "email" | "phone_nr" => {
                    query_string.push_str(&format!(" ORDER BY {} {}", sort_by, order));
                }
                _ => return Err(CustomErrors::ValidationError("Invalid sort field".to_string())),
            }
        } else {
            query_string.push_str(
                " ORDER BY COALESCE(full_name, company_name) ASC, COALESCE(company_name, full_name) ASC"
            );
        }
    
        let records = sqlx::query_as::<_, LandlordDetails>(&query_string)
            .fetch_all(&state.db)
            .await;
    
        match records {
            Ok(landlords) => Ok(landlords),
            Err(e) => Err(CustomErrors::DatabaseError(e.to_string())),
        }
    }


    pub async fn save_details(
        &self,
        state: Arc<AppState>,
        landlord_details: LandlordDetails,
    ) -> Result<JsonValue, CustomErrors> {
        let id = Uuid::new_v4();
        let now = Utc::now();
        let record = sqlx::query("INSERT INTO landlord_details (landlord_id, landlord_type, title, company_name, full_name, email, phone_nr, status, staff_assigned, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)")
            .bind(&id)
            .bind(&landlord_details.landlord_type)
            .bind(&landlord_details.title)
            .bind(&landlord_details.company_name)
            .bind(&landlord_details.full_name)
            .bind(&landlord_details.email)
            .bind(&landlord_details.phone_nr)
            .bind(&landlord_details.status)
            .bind(&landlord_details.staff_assigned)
            .bind(&now)
            .bind(&now)
            .execute(&state.db)
            .await;
        match record {
            Ok(_) => {
                Ok(json!({"status": "success", "message": "Landlord details saved successfully"}))
            }
            Err(e) => Err(CustomErrors::DatabaseError(e.to_string())),
        }
    }
}
