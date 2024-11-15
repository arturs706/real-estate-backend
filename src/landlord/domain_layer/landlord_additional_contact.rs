use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;

use super::landlord_details::LandlordTitle;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct LandlordAdditionalContact {
    pub contact_id: Uuid,
    pub landlord_id: Uuid,
    pub title: Option<LandlordTitle>,
    pub full_name: String,
    pub email: Option<String>,
    pub mobile_phone: Option<String>,
    pub alternative_phone: Option<String>,
    pub is_primary_contact: bool,
    pub notes: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>
}
