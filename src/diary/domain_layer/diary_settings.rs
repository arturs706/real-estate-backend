use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize, FromRow, Debug)]
pub struct DiarySettings {
    pub diary_id: Option<Uuid>,
    pub staff_id: Uuid,
    pub diary_colour: Option<String>,
    pub popup_notifi_en: Option<bool>,
    pub email_notifi_en: Option<bool>,
    pub updated_at: Option<DateTime<Utc>>,
}
