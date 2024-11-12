use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use sqlx::types::JsonValue;
use sqlx::prelude::FromRow;
use std::str::FromStr;
use std::fmt;

#[derive(Clone, Serialize, Deserialize, Debug, sqlx::Type)]
#[sqlx(type_name = "registration_step ", rename_all = "lowercase")]
pub enum RegistrationStep {
    Basicinfo,
    Contactdetails,
    Address,
    Bankdetails,
    Lettingspreferences,
    Completed
}


#[derive(Clone, Serialize, Deserialize, FromRow, Debug)]
pub struct RegistrationProgress {
    pub registration_id: Uuid,
    pub landlord_id: Option<Uuid>,
    pub current_step: RegistrationStep,
    pub registration_data: JsonValue,
    pub expires_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Serialize, Deserialize, FromRow, Debug)]

pub struct RegistrationSummary {
    pub registration_id: Uuid,
    pub user_id: Uuid,
    pub current_step: RegistrationStep,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

impl Default for RegistrationStep {
    fn default() -> Self {
        RegistrationStep::Basicinfo
    }
}

impl fmt::Display for RegistrationStep {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RegistrationStep::Basicinfo => write!(f, "basicinfo"),
            RegistrationStep::Contactdetails => write!(f, "general"),
            RegistrationStep::Address => write!(f, "address"),
            RegistrationStep::Bankdetails => write!(f, "bankdetails"),
            RegistrationStep::Lettingspreferences => write!(f, "lettingspreferences"),
            RegistrationStep::Completed => write!(f, "completed"),
        }
    }
}

// Custom error type for FromStr implementation
#[derive(Debug)]
pub struct ParseRegistrationStepError;

impl fmt::Display for ParseRegistrationStepError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid registration step")
    }
}

// Implement FromStr
impl FromStr for RegistrationStep {
    type Err = ParseRegistrationStepError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "basicinfo" => Ok(RegistrationStep::Basicinfo),
            "general" => Ok(RegistrationStep::Contactdetails),
            "address" => Ok(RegistrationStep::Address),
            "bankdetails" => Ok(RegistrationStep::Bankdetails),
            "lettingspreferences" => Ok(RegistrationStep::Lettingspreferences),
            "completed" => Ok(RegistrationStep::Completed),
            _ => Err(ParseRegistrationStepError),
        }
    }
}