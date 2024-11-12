use chrono::{DateTime, Utc};
use serde::Serialize;
use super::{landlord_details::BasicInfoRequest, landlord_general::LandlordGeneralRequest, landlord_registration_progress::RegistrationStep, landlords_address::AddressRequest, landlords_bank_details::BankDetailsRequest, landlords_lettings_management::LettingsPreferencesRequest};

#[derive(Serialize)]
pub struct StructuredRegistrationData {
    pub basic_info: Option<BasicInfoRequest>,
    pub general_info: Option<LandlordGeneralRequest>,
    pub address: Option<AddressRequest>,
    pub bank_details: Option<BankDetailsRequest>,
    pub lettings_preferences: Option<LettingsPreferencesRequest>,
    pub current_step: RegistrationStep,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

