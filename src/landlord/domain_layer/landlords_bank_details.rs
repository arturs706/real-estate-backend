use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize, FromRow, Debug)]
pub struct LandlordsBankDetails {
    pub id: Uuid,
    pub landlord_id: Uuid,
    pub account_name: String,
    pub account_number: String,
    pub sort_code: String,
    pub iban: Option<String>,
    pub bic: Option<String>,
    pub is_primary: bool,
}

#[derive(Serialize, Deserialize)]
pub struct BankDetailsRequest {
    pub account_name: String,
    pub account_number: String,
    pub sort_code: String,
    pub iban: Option<String>,
    pub bic: Option<String>,
}