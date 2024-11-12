use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditTrail {
    pub entry_id: Uuid,
    pub user_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub description: String,
    pub action_type: String,
    pub ip_address: Option<String>,
}
