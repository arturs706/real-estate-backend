use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Note {
    pub note_id: Uuid,
    pub user_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub note_text: String,
}
