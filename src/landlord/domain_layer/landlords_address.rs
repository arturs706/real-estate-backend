use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize, FromRow, Debug)]
pub struct LandlordsAddress {
    pub id: Uuid,
    pub landlord_id: Uuid,
    pub address_line_1: String,
    pub address_line_2: Option<String>,
    pub city: String,
    pub county: Option<String>,
    pub postcode: String,
    pub country: String,
    pub is_primary: bool,
}

#[derive(Serialize, Deserialize)]
pub struct AddressRequest {
    pub address_line_1: String,
    pub address_line_2: Option<String>,
    pub city: String,
    pub county: Option<String>,
    pub postcode: String,
    pub country: String,
}