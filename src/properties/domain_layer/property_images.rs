use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;


#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PropertyImages {
    pub image_list_id: Uuid,
    pub property_id: Uuid,
    pub image_urls: Vec<String>,
    pub image_descriptions: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
