use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{self, FromRow};
use uuid::Uuid;


#[derive(Clone, Serialize, Deserialize, Debug, sqlx::Type)]
#[sqlx(type_name = "property_status", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]

pub enum PropertyStatus {
    Available,
    LetAgreed,
    Let,
    Withdrawn,
    Unavailable,
    Maintenance,
}




#[derive(Clone, Serialize, Deserialize, Debug, sqlx::Type)]
#[sqlx(type_name = "property_type", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum PropertyType {
    House,
    Flat,
    Apartment,
    Bungalow,
    Maisonette,
    Studio,
    Cottage,
    Terraced,
    SemiDetached,
    Detached,
}



#[derive(Clone, Serialize, Deserialize, Debug, sqlx::Type)]
#[sqlx(type_name = "letting_classification", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")] 

pub enum LettingClassification {
    Residential,
    Commercial,
    Student,
    ShortTerm,
    Holiday,
    Hmo,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PropertyCore {
    pub property_id: Option<Uuid>,
    pub status: PropertyStatus,
    pub property_type: PropertyType,
    pub letting_classification: LettingClassification,
    pub staff_assigned: Option<Uuid>, 
    pub landlord_id: Option<Uuid>,
    pub date_available: Option<NaiveDate>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}