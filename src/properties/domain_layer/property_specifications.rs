use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(Clone, Serialize, Deserialize, Debug, sqlx::Type)]
#[sqlx(type_name = "furnished_status", rename_all = "lowercase")]
pub enum FurnishedStatus {
    Unfurnished,
    PartFurnished,
    FullyFurnished,
}


#[derive(Debug)]
pub struct PropertySpecifications {
    pub spec_id: Uuid,
    pub property_id: Uuid, 
    pub bedrooms: i32,
    pub receptions: i32,
    pub bathrooms: i32,
    pub is_hmo: bool, 
    pub furnished_status: FurnishedStatus,
    pub floor_area_size: Option<f64>,
    pub floor_area_unit: Option<String>, 
    pub created_at: DateTime<Utc>, 
    pub updated_at: DateTime<Utc>, 
}