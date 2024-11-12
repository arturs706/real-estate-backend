use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize, Debug, sqlx::Type)]
#[sqlx(type_name = "landlord_property_categories", rename_all = "lowercase")]

pub enum LandlordPropertyCategory {
    Residential,
    Commercial,
}

#[derive(Clone, Serialize, Deserialize, FromRow, Debug)]
pub struct LandlordGeneral {
    pub landlord_id: Uuid,
    pub notes: Option<String>,
    pub registration_number: Option<String>,
    pub do_not_delete_before: Option<chrono::NaiveDate>,
    pub registration_complete: bool,
    pub is_uk_resident: bool,
    pub property_categories: Vec<LandlordPropertyCategory>,
}

#[derive(Serialize, Deserialize)]
pub struct LandlordGeneralRequest {
    pub registration_number: Option<String>,
    pub do_not_delete_before: Option<chrono::NaiveDate>,
    pub registration_complete: bool,
    pub is_uk_resident: bool,
    pub property_categories: Vec<LandlordPropertyCategory>,
}