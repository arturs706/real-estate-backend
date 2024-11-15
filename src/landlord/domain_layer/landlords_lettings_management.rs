use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize, Debug, sqlx::Type)]
#[sqlx(type_name = "landlord_payment_frequency", rename_all = "lowercase")]
pub enum LandlordPaymentFrequency {
    Weekly,
    Monthly,
    Quarterly,
    SemiAnnually,
    Annually,
    Termly,
    TwoWeekly,
    FourWeekly,
}

#[derive(Clone, Serialize, Deserialize, FromRow, Debug)]
pub struct LandlordsLettingsManagement {
    pub id: Uuid,
    pub landlord_id: Uuid,
    pub payment_frequency: LandlordPaymentFrequency,
    pub is_exempt_from_nrl_tax: bool,
    pub nrl_exemption_reference: Option<String>,
    pub is_exempt_from_vat: bool,
    pub vat_number: Option<String>,
    pub ni_number: Option<String>,
    pub unique_taxpayer_reference: Option<String>,
    pub statement_template_override: Option<String>,
    pub statement_email_subject_override: Option<String>,
    pub statement_payment_ref: Option<String>,
    pub accountant_email: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct LettingsPreferencesRequest {
    pub payment_frequency: LandlordPaymentFrequency,
    pub is_exempt_from_nrl_tax: bool,
    pub nrl_exemption_reference: Option<String>,
    pub is_exempt_from_vat: bool,
    pub vat_number: Option<String>,
    pub ni_number: Option<String>,
    pub unique_taxpayer_reference: Option<String>,
}
