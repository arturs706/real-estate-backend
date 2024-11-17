use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize, Debug, sqlx::Type)]
#[sqlx(type_name = "letting_service_type", rename_all = "lowercase")]
pub enum LettingServiceTypeEnum {
    TenantFindOnly,
    FullManagement,
    RentCollection,
    RentGuarantee,
    Maintenance,
    PropertyLetting,
    PropertyManagement,
    EvictionService,
    TenantScreening,
    MarketAppraisal,
}

#[derive(Clone, Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct PropertyService {
    pub service_id: Uuid,
    pub property_id: Uuid,
    pub service_type: LettingServiceTypeEnum,
    pub letting_fee_percentage: Option<f64>,
    pub letting_fee_amount: Option<f64>,
    pub management_fee_percentage: Option<f64>,
    pub management_fee_amount: Option<f64>,
    pub created_at: DateTime<Utc>
}
