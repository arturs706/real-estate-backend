use uuid::Uuid;
use chrono::DateTime;
use chrono::Utc;

#[derive(Debug)]
pub struct PropertyRentalInfo {
    pub rental_info_id: Uuid,
    pub property_id: Uuid, 
    pub rent_amount: Option<f64>,
    pub rent_poa: bool, 
    pub valuation_rent_min: Option<f64>,
    pub valuation_rent_max: Option<f64>,
    pub minimum_rent: Option<f64>,
    pub deposit: Option<f64>,
    pub holding_deposit: Option<f64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}