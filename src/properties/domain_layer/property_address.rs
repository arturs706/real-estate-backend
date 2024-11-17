use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;


#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PropertyAddress {
    pub address_id: Uuid,
    pub property_id: Uuid, 
    pub display_address: Option<String>, 
    pub address_line1: String, 
    pub address_line2: Option<String>, 
    pub town_city: String, 
    pub county: Option<String>, 
    pub postcode: String,
    pub country: String, 
    pub searchable_area: Option<String>,
    pub created_at: DateTime<Utc>, 
    pub updated_at: DateTime<Utc>
}
