#![allow(dead_code)]

use crate::{
    properties::domain_layer::property_address::PropertyAddress,
    AppState,
};
use actix_web::web::Json;
use uuid::Uuid;
use std::sync::Arc;

pub struct PropertyAddressRepository {}

impl PropertyAddressRepository {
    pub fn new() -> Self {
        PropertyAddressRepository {}
    }

    // Save a property address (insert or update)
    pub async fn save(&self, address: PropertyAddress, state: Arc<AppState>) -> Result<PropertyAddress, Json<String>> {
        let query = r#"
            INSERT INTO property_address (address_id, property_id, display_address, address_line1, address_line2, town_city, county, postcode, country, searchable_area, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            ON CONFLICT (address_id) 
            DO UPDATE 
            SET property_id = $2, display_address = $3, address_line1 = $4, address_line2 = $5, town_city = $6, county = $7, postcode = $8, country = $9, searchable_area = $10, updated_at = $12
            RETURNING *;
        "#;

        let result = sqlx::query_as::<_, PropertyAddress>(query)
            .bind(address.address_id)
            .bind(address.property_id)
            .bind(address.display_address)
            .bind(address.address_line1)
            .bind(address.address_line2)
            .bind(address.town_city)
            .bind(address.county)
            .bind(address.postcode)
            .bind(address.country)
            .bind(address.searchable_area)
            .bind(address.created_at)
            .bind(address.updated_at)
            .fetch_one(&state.db)
            .await;

        match result {
            Ok(saved_address) => Ok(saved_address),
            Err(e) => Err(Json(e.to_string())),
        }
    }

    // Get all property addresses
    pub async fn get_all(&self, state: Arc<AppState>) -> Result<Vec<PropertyAddress>, Json<String>> {
        let result = sqlx::query_as::<_, PropertyAddress>("SELECT * FROM property_address")
            .fetch_all(&state.db)
            .await;

        match result {
            Ok(addresses) => Ok(addresses),
            Err(e) => Err(Json(e.to_string())),
        }
    }

    // Get one property address by its ID
    pub async fn get_one_by_id(&self, address_id: Uuid, state: Arc<AppState>) -> Result<PropertyAddress, Json<String>> {
        let result = sqlx::query_as::<_, PropertyAddress>("SELECT * FROM property_address WHERE address_id = $1")
            .bind(address_id)
            .fetch_one(&state.db)
            .await;

        match result {
            Ok(address) => Ok(address),
            Err(e) => Err(Json(e.to_string())),
        }
    }

    // Get property addresses by property_id
    pub async fn get_one_by_property_id(&self, property_id: Uuid, state: Arc<AppState>) -> Result<Vec<PropertyAddress>, Json<String>> {
        let result = sqlx::query_as::<_, PropertyAddress>("SELECT * FROM property_address WHERE property_id = $1")
            .bind(property_id)
            .fetch_all(&state.db)
            .await;

        match result {
            Ok(addresses) => Ok(addresses),
            Err(e) => Err(Json(e.to_string())),
        }
    }
}
