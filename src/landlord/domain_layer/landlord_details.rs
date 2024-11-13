use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize, Debug, sqlx::Type)]
#[sqlx(type_name = "landlord_status", rename_all = "lowercase")]
pub enum LandlordStatus {
    Active,
    Inactive,
}

#[derive(Clone, Serialize, Deserialize, Debug, sqlx::Type)]
#[sqlx(type_name = "landlord_type_enum", rename_all = "lowercase")]
pub enum LandlordTypeEnum {
    Private,
    Company,
}

#[derive(Clone, Serialize, Deserialize, Debug, sqlx::Type)]
#[sqlx(type_name = "landlord_title_enum", rename_all = "lowercase")]
pub enum LandlordTitle {
    Mr,
    Mrs,
    Miss,
    Ms,
    Dr,
    Prof,
    Rev,
    Other
}


#[derive(Clone, Serialize, Deserialize, FromRow, Debug)]
pub struct LandlordDetails {
    pub landlord_id: Uuid,
    pub title: Option<LandlordStatus>,
    pub full_name: Option<String>,
    pub email: String,
    pub company_name: Option<String>,
    pub mobile_phone: String,
    pub alternative_phone: Option<String>,
    pub additional_contact: Option<String>,
    pub landlord_type: LandlordTypeEnum,
    pub status: LandlordStatus,
    pub staff_added: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
pub struct BasicInfoRequest {
    pub full_name: Option<String>,
    pub title: Option<LandlordStatus>,
    pub email: String,
    pub company_name: Option<String>,
    pub mobile_phone: String,
    pub landlord_type: LandlordTypeEnum,
    pub alternative_phone: Option<String>,
    pub additional_contact: Option<String>,

}

