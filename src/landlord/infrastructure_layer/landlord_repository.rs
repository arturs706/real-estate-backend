use actix_web::error::ResponseError;
use chrono::{Duration, Utc};
use derive_more::Display;
use serde::Serialize;
use uuid::Uuid;
use std::sync::Arc;
use sqlx::{postgres::{PgHasArrayType, PgRow}, types::JsonValue};
use sqlx::Row;
use serde_json::Value;
use crate::{
    landlord::domain_layer::{landlord_details::{BasicInfoRequest, LandlordDetails}, landlord_general::{LandlordGeneralRequest, LandlordPropertyCategory}, landlord_registration_progress::{RegistrationProgress, RegistrationStep, RegistrationSummary}, landlord_structured_registration_data::StructuredRegistrationData, landlords_address::AddressRequest, landlords_bank_details::BankDetailsRequest, landlords_lettings_management::LettingsPreferencesRequest}, AppState
};

#[derive(Debug, Display, Serialize)]
pub enum CustomErrors {
    #[display(fmt = "Database error: {}", _0)]
    DatabaseError(String),
    #[display(fmt = "Not authorized")]
    NotAuthorized,
}

impl ResponseError for CustomErrors {}

impl PgHasArrayType for LandlordPropertyCategory {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("_landlord_property_categories")
    }
}

pub struct LandlordRepository {}

impl LandlordRepository {
    pub fn new() -> Self {
        LandlordRepository {}
    }

    pub async fn get_all(&self, state: Arc<AppState>) -> Result<Vec<LandlordDetails>, CustomErrors> {
        let records = sqlx::query_as::<_, LandlordDetails>("SELECT * FROM landlord_details")
            .fetch_all(&state.db)
            .await;

        match records {
            Ok(users) => Ok(users),
            Err(e) => Err(CustomErrors::DatabaseError(e.to_string())),
        }
    }


    pub async fn start_or_resume_registration(&self, state: Arc<AppState>, user_id: Option<Uuid>) -> Result<RegistrationProgress, CustomErrors> {
        if let Some(user_id) = user_id {
            let existing = sqlx::query_as::<_, RegistrationProgress>(
                "SELECT * FROM landlord_registration_progress 
                WHERE registration_data->>'user_id' = $1 
                AND completed_at IS NULL 
                AND expires_at > CURRENT_TIMESTAMP"
            )
            .bind(user_id)
            .fetch_optional(&state.db)
            .await
            .map_err(|e| CustomErrors::DatabaseError(e.to_string()))?;

            if let Some(progress) = existing {
                return Ok(progress);
            }
        }

        let registration_id = Uuid::new_v4();
        let expires_at = Utc::now() + Duration::days(7); // Registration expires in 7 days

        let progress = sqlx::query_as::<_, RegistrationProgress>(
            "INSERT INTO landlord_registration_progress 
            (registration_id, current_step, registration_data, expires_at) 
            VALUES ($1, $2, $3, $4) 
            RETURNING *"
        )
        .bind(registration_id)
        .bind(RegistrationStep::Basicinfo)
        .bind(serde_json::Value::Object(serde_json::Map::new()))
        .bind(expires_at)
        .fetch_one(&state.db)
        .await
        .map_err(|e| CustomErrors::DatabaseError(e.to_string()))?;
        Ok(progress)
    }

    pub async fn save_registration_step(
        &self,
        state: Arc<AppState>,
        registration_id: Uuid,
        step: RegistrationStep,
        step_data: JsonValue,
    ) -> Result<(), CustomErrors> {
        let result = sqlx::query(
            "UPDATE landlord_registration_progress 
             SET current_step = $1, 
                 registration_data = registration_data || $2,
                 updated_at = CURRENT_TIMESTAMP
             WHERE registration_id = $3 
             AND expires_at > CURRENT_TIMESTAMP",
        )
        .bind(step)
        .bind(step_data)
        .bind(registration_id)
        .execute(&state.db)
        .await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(CustomErrors::DatabaseError(e.to_string())),
        }
    }

    pub async fn complete_registration(
        &self,
        state: Arc<AppState>,
        registration_id: Uuid,
    ) -> Result<Uuid, CustomErrors> {
        // Start transaction
        let mut tx = state.db.begin().await.map_err(|e| CustomErrors::DatabaseError(e.to_string()))?;
    
        // Get registration data
        let registration = sqlx::query_as::<_, RegistrationProgress>(
            "SELECT * FROM landlord_registration_progress WHERE registration_id = $1",
        )
        .bind(registration_id)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| CustomErrors::DatabaseError(e.to_string()))?;
    
        // Create landlord details
        let basic_info: BasicInfoRequest = serde_json::from_value(registration.registration_data["basicinfo"].clone())
            .map_err(|e| CustomErrors::DatabaseError(e.to_string()))?;
        let landlord_id = Uuid::new_v4();
    
        // Insert landlord details
        sqlx::query(
            "INSERT INTO landlord_details 
             (landlord_id, title, full_name, email, company_name, mobile_phone, landlord_type, status, 
              alternative_phone, additional_contact)
             VALUES ($1, $2, $3, $4, $5, $6, $7, 'active', $8, $9)",
        )
        .bind(landlord_id)
        .bind(basic_info.title)
        .bind(basic_info.full_name)
        .bind(basic_info.email)
        .bind(basic_info.company_name)
        .bind(basic_info.mobile_phone)
        .bind(basic_info.landlord_type)
        .bind(basic_info.alternative_phone)
        .bind(basic_info.additional_contact)
        .execute(&mut *tx)
        .await
        .map_err(|e| CustomErrors::DatabaseError(e.to_string()))?;
    
        let general_info: LandlordGeneralRequest = serde_json::from_value(registration.registration_data["general"].clone())
            .map_err(|e| CustomErrors::DatabaseError(e.to_string()))?;
        
        sqlx::query(
            "INSERT INTO landlord_general 
             (landlord_id, registration_number, do_not_delete_before, registration_complete, 
              is_uk_resident, property_categories)
             VALUES ($1, $2, $3, $4, $5, $6)",
        )
        .bind(landlord_id)
        .bind(general_info.registration_number)
        .bind(general_info.do_not_delete_before)
        .bind(general_info.registration_complete)
        .bind(general_info.is_uk_resident)
        .bind(&general_info.property_categories)
        .execute(&mut *tx)
        .await
        .map_err(|e| CustomErrors::DatabaseError(e.to_string()))?;
    
        // Insert address
        let address: AddressRequest = serde_json::from_value(registration.registration_data["address"].clone())
            .map_err(|e| CustomErrors::DatabaseError(e.to_string()))?;
        
        sqlx::query(
            "INSERT INTO landlords_address 
             (landlord_id, address_line_1, address_line_2, city, county, postcode, country, is_primary)
             VALUES ($1, $2, $3, $4, $5, $6, $7, true)",
        )
        .bind(landlord_id)
        .bind(address.address_line_1)
        .bind(address.address_line_2)
        .bind(address.city)
        .bind(address.county)
        .bind(address.postcode)
        .bind(address.country)
        .execute(&mut *tx)
        .await
        .map_err(|e| CustomErrors::DatabaseError(e.to_string()))?;
    
        // Insert bank details
        let bank_details: BankDetailsRequest = serde_json::from_value(registration.registration_data["bankdetails"].clone())
            .map_err(|e| CustomErrors::DatabaseError(e.to_string()))?;
        
        sqlx::query(
            "INSERT INTO landlords_bank_details 
             (landlord_id, account_name, account_number, sort_code, iban, bic, is_primary)
             VALUES ($1, $2, $3, $4, $5, $6, true)",
        )
        .bind(landlord_id)
        .bind(bank_details.account_name)
        .bind(bank_details.account_number)
        .bind(bank_details.sort_code)
        .bind(bank_details.iban)
        .bind(bank_details.bic)
        .execute(&mut *tx)
        .await
        .map_err(|e| CustomErrors::DatabaseError(e.to_string()))?;
    
        // Insert lettings preferences
        let lettings_preferences: LettingsPreferencesRequest = serde_json::from_value(registration.registration_data["lettingspreferences"].clone())
            .map_err(|e| CustomErrors::DatabaseError(e.to_string()))?;
        
        sqlx::query(
            "INSERT INTO landlords_lettings_management 
             (landlord_id, payment_frequency, is_exempt_from_nrl_tax, nrl_exemption_reference,
              is_exempt_from_vat, vat_number, ni_number, unique_taxpayer_reference)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
        )
        .bind(landlord_id)
        .bind(lettings_preferences.payment_frequency)
        .bind(lettings_preferences.is_exempt_from_nrl_tax)
        .bind(lettings_preferences.nrl_exemption_reference)
        .bind(lettings_preferences.is_exempt_from_vat)
        .bind(lettings_preferences.vat_number)
        .bind(lettings_preferences.ni_number)
        .bind(lettings_preferences.unique_taxpayer_reference)
        .execute(&mut *tx)
        .await
        .map_err(|e| CustomErrors::DatabaseError(e.to_string()))?;
    
        // Update registration progress
        sqlx::query(
            "UPDATE landlord_registration_progress 
             SET landlord_id = $1,
                 current_step = 'completed',
                 completed_at = CURRENT_TIMESTAMP
             WHERE registration_id = $2",
        )
        .bind(landlord_id)
        .bind(registration_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| CustomErrors::DatabaseError(e.to_string()))?;
    
        tx.commit()
            .await
            .map_err(|e| CustomErrors::DatabaseError(e.to_string()))?;
    
        Ok(landlord_id)
    }


    pub async fn get_user_registrations(
        &self,
        state: Arc<AppState>,
        user_id: Uuid,
    ) -> Result<Vec<RegistrationSummary>, CustomErrors> {
        let query = r#"
            SELECT
                registration_id,
                registration_data->>'user_id' as user_id,
                current_step,
                created_at,
                expires_at,
                completed_at
            FROM landlord_registration_progress
            WHERE
                registration_data->>'user_id' = $1
                AND expires_at > CURRENT_TIMESTAMP
            ORDER BY created_at DESC
        "#;

        let rows = sqlx::query(query)
            .bind(user_id)
            .map(|row: PgRow| RegistrationSummary {
                registration_id: row.get("registration_id"),
                user_id: row.get("user_id"),
                current_step: row.get::<String, _>("current_step")
                    .parse()
                    .unwrap_or_default(),
                created_at: row.get("created_at"),
                expires_at: row.get("expires_at"),
                completed_at: row.get("completed_at"),
            })
            .fetch_all(&state.db)
            .await
            .map_err(|e| CustomErrors::DatabaseError(e.to_string()))?;

        Ok(rows)
    }

    pub async fn get_registration_details(
        &self,
        state: Arc<AppState>,
        registration_id: Uuid,
    ) -> Result<Option<RegistrationProgress>, CustomErrors> {
        let query = r#"
            SELECT
                registration_id,
                landlord_id,
                current_step,
                registration_data,
                expires_at,
                completed_at,
                created_at,
                updated_at
            FROM landlord_registration_progress
            WHERE
                registration_id = $1
                AND expires_at > CURRENT_TIMESTAMP
        "#;

        let row = sqlx::query(query)
            .bind(registration_id)
            .map(|row: PgRow| RegistrationProgress {
                registration_id: row.get("registration_id"),
                landlord_id: row.get("landlord_id"),
                current_step: row.get::<String, _>("current_step")
                    .parse()
                    .unwrap_or_default(),
                registration_data: row.get("registration_data"),
                expires_at: row.get("expires_at"),
                completed_at: row.get("completed_at"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            })
            .fetch_optional(&state.db)
            .await
            .map_err(|e| CustomErrors::DatabaseError(e.to_string()))?;

        Ok(row)
    }

    pub async fn get_structured_registration_data(
        &self,
        state: Arc<AppState>,
        registration_id: Uuid,
    ) -> Result<Option<StructuredRegistrationData>, CustomErrors> {
        if let Some(registration) = self.get_registration_details(state.clone(), registration_id).await? {
            let get_json_data = |key: &str| -> Value {
                registration.registration_data
                    .get(key)
                    .cloned()
                    .unwrap_or_default()
            };

            let structured_data = StructuredRegistrationData {
                basic_info: serde_json::from_value(get_json_data("basicinfo"))
                    .unwrap_or_default(),
                general_info: serde_json::from_value(get_json_data("general"))
                    .unwrap_or_default(),
                address: serde_json::from_value(get_json_data("address"))
                    .unwrap_or_default(),
                bank_details: serde_json::from_value(get_json_data("bankdetails"))
                    .unwrap_or_default(),
                lettings_preferences: serde_json::from_value(get_json_data("lettingspreferences"))
                    .unwrap_or_default(),
                current_step: registration.current_step,
                created_at: registration.created_at,
                expires_at: registration.expires_at,
                completed_at: registration.completed_at,
            };

            Ok(Some(structured_data))
        } else {
            Ok(None)
        }
    }

}
