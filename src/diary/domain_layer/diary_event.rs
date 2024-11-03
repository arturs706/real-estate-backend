use chrono::{DateTime, NaiveDate, NaiveTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize, FromRow, Debug)]
pub struct Event {
    pub id: Option<Uuid>,
    pub external_id: String,
    pub event_type: EventType,
    pub date: NaiveDate,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub title: Option<String>,
    pub description: Option<String>,
    pub created_by: Uuid,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Serialize, Deserialize, Debug, sqlx::Type)]
#[sqlx(type_name = "event_type", rename_all = "lowercase")]
pub enum EventType {
    Viewing,
    Appointment,
    Inspection,
    Note,
    SickLeave,
    StaffMeeting,
    Valuation,
    Callback,
    Maintenance,
    PublicHoliday,
    StaffHoliday,
    Training,
}
