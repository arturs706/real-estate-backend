use chrono::{DateTime, NaiveDate, NaiveTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize, Debug, FromRow)]
pub struct CreateEventRequest {
    pub event: Event,
    pub details: EventDetails,
}
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

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(tag = "event_type", content = "data")]
pub enum EventDetails {
    Viewing(ViewingDetails),
    Appointment(AppointmentDetails),
    Inspection(InspectionDetails),
    SickLeave(LeaveDetails),
    StaffMeeting(MeetingDetails),
    Valuation(ValuationDetails),
    Callback(CallbackDetails),
    Maintenance(MaintenanceDetails),
    StaffHoliday(StaffHolidayDetails),
    Training(TrainingDetails),
    PublicHoliday(PublicHolidayDetails),
    Note(NoteDetails),
}

#[derive(Clone, Serialize, Deserialize, Debug, FromRow)]
pub struct StaffHolidayDetails {
    pub staff_member: Uuid,
    pub holiday_type: Option<String>,
    pub is_half_day: Option<bool>,
    pub approval_status: Option<String>,
    pub approved_by: Option<Uuid>,
    pub approval_date: Option<OffsetDateTime>,
    pub remaining_days: Option<f64>,
}

#[derive(Clone, Serialize, Deserialize, Debug, FromRow)]
pub struct TrainingDetails {
    pub training_title: String,
    pub location: Option<String>,
    pub lead_staff: Option<Uuid>,
    pub attendees: Option<Vec<Uuid>>,
    pub additional_attendees: Option<Vec<String>>,
    pub training_type: Option<String>,
    pub training_status: Option<String>,
    pub materials_url: Option<String>,
    pub prerequisites: Option<String>,
    pub attendance_confirmed: Option<bool>,
    pub certificates_issued: Option<bool>,
}
#[derive(Clone, Serialize, Deserialize, Debug, FromRow)]
pub struct PublicHolidayDetails {
    pub holiday_name: String,
    pub region: Option<String>,
    pub affects_all_staff: Option<bool>,
    pub affected_departments: Option<Vec<String>>,
    pub is_bank_holiday: Option<bool>,
    pub office_status: Option<String>,
    pub custom_working_hours: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug, FromRow)]
pub struct NoteDetails {
    pub note_type: String,
    pub assigned_staff: Option<Vec<Uuid>>,
    pub is_private: Option<bool>,
    pub category: Option<String>,
    pub priority: Option<String>,
    pub related_entity_type: Option<String>,
    pub related_entity_id: Option<String>,
    pub status: Option<String>,
    pub completion_date: Option<OffsetDateTime>,
    pub completed_by: Option<Uuid>,
}

#[derive(Clone, Serialize, Deserialize, Debug, FromRow)]
pub struct ViewingDetails {
    pub property_id: String,
    pub client_name: String,
    pub contact_number: String,
    pub viewing_type: String,
    pub notification_length: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug, FromRow)]
pub struct AppointmentDetails {
    pub location: Option<String>,
    pub property_id: Option<String>,
    pub is_private: Option<bool>,
    pub notification: Option<bool>,
    pub is_recurring: Option<bool>,
    pub recurrence_pattern: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug, FromRow)]
pub struct InspectionDetails {
    pub property_id: String,
    pub contractor: String,
    pub notification: Option<bool>,
}

#[derive(Clone, Serialize, Deserialize, Debug, FromRow)]
pub struct LeaveDetails {
    pub staff_member: String,
    pub is_half_day: Option<bool>,
}

#[derive(Clone, Serialize, Deserialize, Debug, FromRow)]
pub struct MeetingDetails {
    pub location: Option<String>,
    pub is_recurring: Option<bool>,
    pub recurrence_pattern: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug, FromRow)]
pub struct ValuationDetails {
    pub property_id: String,
    pub client_name: String,
    pub contact_number: String,
    pub notification: Option<bool>,
}

#[derive(Clone, Serialize, Deserialize, Debug, FromRow)]
pub struct CallbackDetails {
    pub contact_name: String,
    pub phone_number: String,
    pub is_urgent: Option<bool>,
}

#[derive(Clone, Serialize, Deserialize, Debug, FromRow)]
pub struct MaintenanceDetails {
    pub property_id: String,
    pub contractor: String,
    pub notification: Option<bool>,
}

#[derive(Deserialize)]
pub struct DateQueryParams {
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
}
