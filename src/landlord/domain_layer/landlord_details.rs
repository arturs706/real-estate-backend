use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize, Debug, sqlx::Type)]
#[sqlx(type_name = "landlord_status", rename_all = "lowercase")]
#[serde(rename_all = "camelCase")]
pub enum LandlordStatus {
    Active,
    Inactive,
}

#[derive(Clone, Serialize, Deserialize, Debug, sqlx::Type)]
#[sqlx(type_name = "landlord_type_enum", rename_all = "lowercase")]
#[serde(rename_all = "camelCase")] 
pub enum LandlordTypeEnum {
    Private,
    Company,
}

#[derive(Clone, Serialize, Deserialize, Debug, sqlx::Type)]
#[sqlx(type_name = "landlord_title_enum", rename_all = "lowercase")]
#[serde(rename_all = "camelCase")]
pub enum LandlordTitle {
    Mr,
    Mrs,
    Miss,
    Ms,
    Dr,
    Prof,
    Rev,
    Other,
}


#[derive(Clone, Serialize, Deserialize, FromRow, Debug)]
pub struct LandlordDetails {
    pub landlord_id: Option<Uuid>,
    pub landlord_type: LandlordTypeEnum,
    pub title: Option<LandlordTitle>,
    pub company_name: Option<String>,
    pub full_name: Option<String>,
    pub email: Option<String>,
    pub phone_nr: String,
    pub status: LandlordStatus,
    pub staff_assigned: Option<Uuid>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>
}


#[derive(serde::Deserialize)]
pub struct LandlordQueryParams {
    pub status: Option<String>, 
    pub sort_by: Option<String>,
    pub order: Option<String>, 
}