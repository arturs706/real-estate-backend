use crate::{
    diary::domain_layer::diary_event_types::{
        AppointmentDetails, CallbackDetails, Event, EventDetails, EventType, InspectionDetails,
        LeaveDetails, MaintenanceDetails, MeetingDetails, NoteDetails, PublicHolidayDetails,
        StaffHolidayDetails, TrainingDetails, ValuationDetails, ViewingDetails,
    },
    AppState,
};
use actix_web::{dev::Path, error::ResponseError};
use chrono::{DateTime, NaiveDate, Utc};
use derive_more::Display;
use serde::Serialize;
use serde_json::Value;
use sqlx::types::JsonValue;
use sqlx::{FromRow, Row};
use std::sync::Arc;
use uuid::Uuid; // Add Row trait import

use super::custom_error_repo_diary::CustomErrorsDiary;

#[derive(Debug, Display, Serialize)]
pub enum CustomErrors {
    #[display(fmt = "Database error: {}", _0)]
    DatabaseError(String),
    #[display(fmt = "Not found")]
    NotFound,
    #[display(fmt = "Bad request: {}", _0)]
    BadRequest(String),
}

impl ResponseError for CustomErrors {}

pub struct EventRepository {}

impl EventRepository {
    pub fn new() -> Self {
        EventRepository {}
    }

    pub async fn create_event(
        &self,
        state: Arc<AppState>,
        new_event: Event,
        details: EventDetails,
    ) -> Result<Event, CustomErrors> {
        let pool = &state.db;
        let mut tx = pool
            .begin()
            .await
            .map_err(|e| CustomErrors::DatabaseError(e.to_string()))?;

        // Insert main event
        let event = sqlx::query_as::<_, Event>(
    "INSERT INTO events (external_id, event_type, date, start_time, end_time, created_by, title, description)
    VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
    RETURNING *"
    )
    .bind(&new_event.external_id)
    .bind(&new_event.event_type)
    .bind(&new_event.date)
    .bind(&new_event.start_time)
    .bind(&new_event.end_time)
    .bind(&new_event.created_by)
    .bind(&new_event.title)
    .bind(&new_event.description)
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| CustomErrors::DatabaseError(e.to_string()))?;

        // Insert event details based on type
        match details {
            EventDetails::Viewing(details) => {
                sqlx::query(
    "INSERT INTO viewing_details (event_id, property_id, client_name, contact_number, viewing_type, notification_length)
    VALUES ($1, $2, $3, $4, $5, $6)"
    )
    .bind(event.id)
    .bind(&details.property_id)
    .bind(&details.client_name)
    .bind(&details.contact_number)
    .bind(&details.viewing_type)
    .bind(&details.notification_length)
    .execute(&mut *tx)
    .await
    .map_err(|e| CustomErrors::DatabaseError(e.to_string()))?;
            }
            EventDetails::Appointment(details) => {
                sqlx::query(
    "INSERT INTO appointment_details (event_id, location, property_id, is_private, notification, is_recurring, recurrence_pattern)
    VALUES ($1, $2, $3, $4, $5, $6, $7)"
    )
    .bind(event.id)
    .bind(&details.location)
    .bind(&details.property_id)
    .bind(&details.is_private)
    .bind(&details.notification)
    .bind(&details.is_recurring)
    .bind(&details.recurrence_pattern)
    .execute(&mut *tx)
    .await
    .map_err(|e| CustomErrors::DatabaseError(e.to_string()))?;
            }
            EventDetails::Inspection(details) => {
                sqlx::query(
    "INSERT INTO inspection_details (event_id, property_id, contractor, notification)
    VALUES ($1, $2, $3, $4)"
    )
    .bind(event.id)
    .bind(&details.property_id)
    .bind(&details.contractor)
    .bind(&details.notification)
    .execute(&mut *tx)
    .await
    .map_err(|e| CustomErrors::DatabaseError(e.to_string()))?;
            }
            EventDetails::SickLeave(details) => {
                sqlx::query(
                    "INSERT INTO leave_details (event_id, staff_member, is_half_day)
    VALUES ($1, $2, $3)",
                )
                .bind(event.id)
                .bind(&details.staff_member)
                .bind(&details.is_half_day)
                .execute(&mut *tx)
                .await
                .map_err(|e| CustomErrors::DatabaseError(e.to_string()))?;
            }
            EventDetails::StaffMeeting(details) => {
                sqlx::query(
    "INSERT INTO meeting_details (event_id, location, is_recurring, recurrence_pattern)
    VALUES ($1, $2, $3, $4)"
    )
    .bind(event.id)
    .bind(&details.location)
    .bind(&details.is_recurring)
    .bind(&details.recurrence_pattern)
    .execute(&mut *tx)
    .await
    .map_err(|e| CustomErrors::DatabaseError(e.to_string()))?;
            }
            EventDetails::Valuation(details) => {
                sqlx::query(
    "INSERT INTO valuation_details (event_id, property_id, client_name, contact_number, notification)
    VALUES ($1, $2, $3, $4, $5)"
    )
    .bind(event.id)
    .bind(&details.property_id)
    .bind(&details.client_name)
    .bind(&details.contact_number)
    .bind(&details.notification)
    .execute(&mut *tx)
    .await
    .map_err(|e| CustomErrors::DatabaseError(e.to_string()))?;
            }
            EventDetails::Callback(details) => {
                sqlx::query(
                    "INSERT INTO callback_details (event_id, contact_name, phone_number, is_urgent)
    VALUES ($1, $2, $3, $4)",
                )
                .bind(event.id)
                .bind(&details.contact_name)
                .bind(&details.phone_number)
                .bind(&details.is_urgent)
                .execute(&mut *tx)
                .await
                .map_err(|e| CustomErrors::DatabaseError(e.to_string()))?;
            }
            EventDetails::Maintenance(details) => {
                sqlx::query(
    "INSERT INTO maintenance_details (event_id, property_id, contractor, notification)
    VALUES ($1, $2, $3, $4)"
    )
    .bind(event.id)
    .bind(&details.property_id)
    .bind(&details.contractor)
    .bind(&details.notification)
    .execute(&mut *tx)
    .await
    .map_err(|e| CustomErrors::DatabaseError(e.to_string()))?;
            }
            EventDetails::StaffHoliday(details) => {
                sqlx::query(
                    "INSERT INTO staff_holiday_details (
    event_id, staff_member, holiday_type, is_half_day,
    approval_status, approved_by, approval_date, remaining_days
    ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
                )
                .bind(event.id)
                .bind(&details.staff_member)
                .bind(&details.holiday_type)
                .bind(&details.is_half_day)
                .bind(&details.approval_status)
                .bind(&details.approved_by)
                .bind(&details.approval_date)
                .bind(&details.remaining_days)
                .execute(&mut *tx)
                .await
                .map_err(|e| CustomErrors::DatabaseError(e.to_string()))?;
            }

            EventDetails::Training(details) => {
                sqlx::query(
                    "INSERT INTO training_details (
    event_id, training_title, location, lead_staff,
    attendees, additional_attendees, training_type,
    training_status, materials_url, prerequisites,
    attendance_confirmed, certificates_issued
    ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)",
                )
                .bind(event.id)
                .bind(&details.training_title)
                .bind(&details.location)
                .bind(&details.lead_staff)
                .bind(&details.attendees)
                .bind(&details.additional_attendees)
                .bind(&details.training_type)
                .bind(&details.training_status)
                .bind(&details.materials_url)
                .bind(&details.prerequisites)
                .bind(&details.attendance_confirmed)
                .bind(&details.certificates_issued)
                .execute(&mut *tx)
                .await
                .map_err(|e| CustomErrors::DatabaseError(e.to_string()))?;
            }
            EventDetails::PublicHoliday(details) => {
                sqlx::query(
                    "INSERT INTO public_holiday_details (
    event_id, holiday_name, region, affects_all_staff,
    affected_departments, is_bank_holiday, office_status,
    custom_working_hours
    ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
                )
                .bind(event.id)
                .bind(&details.holiday_name)
                .bind(&details.region)
                .bind(&details.affects_all_staff)
                .bind(&details.affected_departments)
                .bind(&details.is_bank_holiday)
                .bind(&details.office_status)
                .bind(&details.custom_working_hours)
                .execute(&mut *tx)
                .await
                .map_err(|e| CustomErrors::DatabaseError(e.to_string()))?;
            }
            EventDetails::Note(details) => {
                sqlx::query(
                    "INSERT INTO note_details (
    event_id, note_type, assigned_staff, is_private,
    category, priority, related_entity_type, related_entity_id,
    status, completion_date, completed_by
    ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)",
                )
                .bind(event.id)
                .bind(&details.note_type)
                .bind(&details.assigned_staff)
                .bind(&details.is_private)
                .bind(&details.category)
                .bind(&details.priority)
                .bind(&details.related_entity_type)
                .bind(&details.related_entity_id)
                .bind(&details.status)
                .bind(&details.completion_date)
                .bind(&details.completed_by)
                .execute(&mut *tx)
                .await
                .map_err(|e| CustomErrors::DatabaseError(e.to_string()))?;
            }
        }

        tx.commit()
            .await
            .map_err(|e| CustomErrors::DatabaseError(e.to_string()))?;
        Ok(event)
    }

    pub async fn get_all_events(&self, state: Arc<AppState>) -> Result<Vec<Event>, CustomErrors> {
        let pool = &state.db;
        sqlx::query_as::<_, Event>("SELECT * FROM events ORDER BY date, start_time")
            .fetch_all(pool)
            .await
            .map_err(|e| CustomErrors::DatabaseError(e.to_string()))
    }

    pub async fn get_event_by_id(
        &self,
        state: Arc<AppState>,
        event_id: Uuid,
    ) -> Result<Event, CustomErrors> {
        let pool = &state.db;
        sqlx::query_as::<_, Event>("SELECT * FROM events WHERE id = $1")
            .bind(event_id)
            .fetch_optional(pool)
            .await
            .map_err(|e| CustomErrors::DatabaseError(e.to_string()))?
            .ok_or(CustomErrors::NotFound)
    }

    pub async fn get_user_events(
        &self,
        state: Arc<AppState>,
        user_id: Uuid,
    ) -> Result<Vec<(Event, EventDetails)>, CustomErrorsDiary> {
        let pool = &state.db;
        let query = r#"
    SELECT
        e.*,
        CASE e.event_type
                   WHEN 'viewing' THEN (
                SELECT jsonb_build_object(
                    'property_id', vd.property_id,
                    'client_name', vd.client_name,
                    'contact_number', vd.contact_number,
                    'viewing_type', vd.viewing_type,
                    'notification_length', vd.notification_length
                )
                FROM viewing_details vd
                WHERE vd.event_id = e.id
            )
            WHEN 'appointment' THEN (
                SELECT jsonb_build_object(
                    'location', ad.location,
                    'property_id', ad.property_id,
                    'is_private', ad.is_private,
                    'notification', ad.notification,
                    'is_recurring', ad.is_recurring,
                    'recurrence_pattern', ad.recurrence_pattern
                )
                FROM appointment_details ad
                WHERE ad.event_id = e.id
            )
            WHEN 'inspection' THEN (
                SELECT jsonb_build_object(
                    'property_id', id.property_id,
                    'contractor', id.contractor,
                    'notification', id.notification
                )
                FROM inspection_details id
                WHERE id.event_id = e.id
            )
            WHEN 'sickleave' THEN (
                SELECT jsonb_build_object(
                    'staff_member', ld.staff_member,
                    'is_half_day', ld.is_half_day
                )
                FROM leave_details ld
                WHERE ld.event_id = e.id
            )
            WHEN 'staffmeeting' THEN (
                SELECT jsonb_build_object(
                    'location', md.location,
                    'is_recurring', md.is_recurring,
                    'recurrence_pattern', md.recurrence_pattern
                )
                FROM meeting_details md
                WHERE md.event_id = e.id
            )
            WHEN 'valuation' THEN (
                SELECT jsonb_build_object(
                    'property_id', vd.property_id,
                    'client_name', vd.client_name,
                    'contact_number', vd.contact_number,
                    'notification', vd.notification
                )
                FROM valuation_details vd
                WHERE vd.event_id = e.id
            )
            WHEN 'callback' THEN (
                SELECT jsonb_build_object(
                    'contact_name', cd.contact_name,
                    'phone_number', cd.phone_number,
                    'is_urgent', cd.is_urgent
                )
                FROM callback_details cd
                WHERE cd.event_id = e.id
            )
            WHEN 'maintenance' THEN (
                SELECT jsonb_build_object(
                    'property_id', md.property_id,
                    'contractor', md.contractor,
                    'notification', md.notification
                )
                FROM maintenance_details md
                WHERE md.event_id = e.id
            )
            WHEN 'staffholiday' THEN (
                SELECT jsonb_build_object(
                    'staff_member', hd.staff_member,
                    'holiday_type', hd.holiday_type,
                    'is_half_day', hd.is_half_day,
                    'approval_status', hd.approval_status,
                    'approved_by', hd.approved_by,
                    'approval_date', hd.approval_date,
                    'remaining_days', hd.remaining_days
                )
                FROM staff_holiday_details hd
                WHERE hd.event_id = e.id
            )
            WHEN 'training' THEN (
                SELECT jsonb_build_object(
                    'training_title', td.training_title,
                    'location', td.location,
                    'lead_staff', td.lead_staff,
                    'attendees', td.attendees,
                    'additional_attendees', td.additional_attendees,
                    'training_type', td.training_type,
                    'training_status', td.training_status,
                    'materials_url', td.materials_url,
                    'prerequisites', td.prerequisites,
                    'attendance_confirmed', td.attendance_confirmed,
                    'certificates_issued', td.certificates_issued
                )
                FROM training_details td
                WHERE td.event_id = e.id
            )
            WHEN 'publicholiday' THEN (
                SELECT jsonb_build_object(
                    'holiday_name', phd.holiday_name,
                    'region', phd.region,
                    'affects_all_staff', phd.affects_all_staff,
                    'affected_departments', phd.affected_departments,
                    'is_bank_holiday', phd.is_bank_holiday,
                    'office_status', phd.office_status,
                    'custom_working_hours', phd.custom_working_hours
                )
                FROM public_holiday_details phd
                WHERE phd.event_id = e.id
            )
            WHEN 'note' THEN (
                SELECT jsonb_build_object(
                    'note_type', nd.note_type,
                    'assigned_staff', nd.assigned_staff,
                    'is_private', nd.is_private,
                    'category', nd.category,
                    'priority', nd.priority,
                    'related_entity_type', nd.related_entity_type,
                    'related_entity_id', nd.related_entity_id,
                    'status', nd.status,
                    'completion_date', nd.completion_date,
                    'completed_by', nd.completed_by
                )
                FROM note_details nd
                WHERE nd.event_id = e.id
            )
        END AS details
    FROM events e
    WHERE e.created_by = $1
    ORDER BY e.date, e.start_time
"#;
        // Rest of the function remains the same...
        let query_builder = sqlx::query(&query).bind(user_id);
        let rows = query_builder
            .fetch_all(pool)
            .await
            .map_err(CustomErrorsDiary::from)?;
        let result = rows
            .into_iter()
            .map(|row| {
                let event = Event {
                    id: row.get("id"),
                    external_id: row.get("external_id"),
                    event_type: row.get("event_type"),
                    date: row.get("date"),
                    start_time: row.get("start_time"),
                    end_time: row.get("end_time"),
                    title: row.get("title"),
                    description: row.get("description"),
                    created_by: row.get("created_by"),
                    created_at: Some(row.get::<DateTime<Utc>, _>("created_at")),
                    updated_at: Some(row.get::<DateTime<Utc>, _>("updated_at")),
                };
                let details_json: JsonValue = row.get("details");
                let details = match event.event_type {
                    EventType::Viewing => serde_json::from_value(details_json)
                        .map(EventDetails::Viewing)
                        .map_err(CustomErrorsDiary::from),
                    EventType::Appointment => serde_json::from_value(details_json)
                        .map(EventDetails::Appointment)
                        .map_err(CustomErrorsDiary::from),
                    EventType::Inspection => serde_json::from_value(details_json)
                        .map(EventDetails::Inspection)
                        .map_err(CustomErrorsDiary::from),
                    EventType::Note => serde_json::from_value(details_json)
                        .map(EventDetails::Note)
                        .map_err(CustomErrorsDiary::from),
                    EventType::SickLeave => serde_json::from_value(details_json)
                        .map(EventDetails::SickLeave)
                        .map_err(CustomErrorsDiary::from),
                    EventType::StaffMeeting => serde_json::from_value(details_json)
                        .map(EventDetails::StaffMeeting)
                        .map_err(CustomErrorsDiary::from),
                    EventType::Valuation => serde_json::from_value(details_json)
                        .map(EventDetails::Valuation)
                        .map_err(CustomErrorsDiary::from),
                    EventType::Callback => serde_json::from_value(details_json)
                        .map(EventDetails::Callback)
                        .map_err(CustomErrorsDiary::from),
                    EventType::Maintenance => serde_json::from_value(details_json)
                        .map(EventDetails::Maintenance)
                        .map_err(CustomErrorsDiary::from),
                    EventType::PublicHoliday => serde_json::from_value(details_json)
                        .map(EventDetails::PublicHoliday)
                        .map_err(CustomErrorsDiary::from),
                    EventType::StaffHoliday => serde_json::from_value(details_json)
                        .map(EventDetails::StaffHoliday)
                        .map_err(CustomErrorsDiary::from),
                    EventType::Training => serde_json::from_value(details_json)
                        .map(EventDetails::Training)
                        .map_err(CustomErrorsDiary::from),
                }?;
                Ok((event, details))
            })
            .collect::<Result<Vec<_>, CustomErrorsDiary>>()?;
        Ok(result)
    }

    pub async fn get_user_events_with_dates(
        &self,
        state: Arc<AppState>,
        user_id: Uuid,
        start_date: Option<NaiveDate>,
        end_date: Option<NaiveDate>,
    ) -> Result<Vec<(Event, EventDetails)>, CustomErrorsDiary> {
        let pool = &state.db;
        let query = r#"
        SELECT
        e.*,
        CASE e.event_type
                   WHEN 'viewing' THEN (
                SELECT jsonb_build_object(
                    'property_id', vd.property_id,
                    'client_name', vd.client_name,
                    'contact_number', vd.contact_number,
                    'viewing_type', vd.viewing_type,
                    'notification_length', vd.notification_length
                )
                FROM viewing_details vd
                WHERE vd.event_id = e.id
            )
            WHEN 'appointment' THEN (
                SELECT jsonb_build_object(
                    'location', ad.location,
                    'property_id', ad.property_id,
                    'is_private', ad.is_private,
                    'notification', ad.notification,
                    'is_recurring', ad.is_recurring,
                    'recurrence_pattern', ad.recurrence_pattern
                )
                FROM appointment_details ad
                WHERE ad.event_id = e.id
            )
            WHEN 'inspection' THEN (
                SELECT jsonb_build_object(
                    'property_id', id.property_id,
                    'contractor', id.contractor,
                    'notification', id.notification
                )
                FROM inspection_details id
                WHERE id.event_id = e.id
            )
            WHEN 'sickleave' THEN (
                SELECT jsonb_build_object(
                    'staff_member', ld.staff_member,
                    'is_half_day', ld.is_half_day
                )
                FROM leave_details ld
                WHERE ld.event_id = e.id
            )
            WHEN 'staffmeeting' THEN (
                SELECT jsonb_build_object(
                    'location', md.location,
                    'is_recurring', md.is_recurring,
                    'recurrence_pattern', md.recurrence_pattern
                )
                FROM meeting_details md
                WHERE md.event_id = e.id
            )
            WHEN 'valuation' THEN (
                SELECT jsonb_build_object(
                    'property_id', vd.property_id,
                    'client_name', vd.client_name,
                    'contact_number', vd.contact_number,
                    'notification', vd.notification
                )
                FROM valuation_details vd
                WHERE vd.event_id = e.id
            )
            WHEN 'callback' THEN (
                SELECT jsonb_build_object(
                    'contact_name', cd.contact_name,
                    'phone_number', cd.phone_number,
                    'is_urgent', cd.is_urgent
                )
                FROM callback_details cd
                WHERE cd.event_id = e.id
            )
            WHEN 'maintenance' THEN (
                SELECT jsonb_build_object(
                    'property_id', md.property_id,
                    'contractor', md.contractor,
                    'notification', md.notification
                )
                FROM maintenance_details md
                WHERE md.event_id = e.id
            )
            WHEN 'staffholiday' THEN (
                SELECT jsonb_build_object(
                    'staff_member', hd.staff_member,
                    'holiday_type', hd.holiday_type,
                    'is_half_day', hd.is_half_day,
                    'approval_status', hd.approval_status,
                    'approved_by', hd.approved_by,
                    'approval_date', hd.approval_date,
                    'remaining_days', hd.remaining_days
                )
                FROM staff_holiday_details hd
                WHERE hd.event_id = e.id
            )
            WHEN 'training' THEN (
                SELECT jsonb_build_object(
                    'training_title', td.training_title,
                    'location', td.location,
                    'lead_staff', td.lead_staff,
                    'attendees', td.attendees,
                    'additional_attendees', td.additional_attendees,
                    'training_type', td.training_type,
                    'training_status', td.training_status,
                    'materials_url', td.materials_url,
                    'prerequisites', td.prerequisites,
                    'attendance_confirmed', td.attendance_confirmed,
                    'certificates_issued', td.certificates_issued
                )
                FROM training_details td
                WHERE td.event_id = e.id
            )
            WHEN 'publicholiday' THEN (
                SELECT jsonb_build_object(
                    'holiday_name', phd.holiday_name,
                    'region', phd.region,
                    'affects_all_staff', phd.affects_all_staff,
                    'affected_departments', phd.affected_departments,
                    'is_bank_holiday', phd.is_bank_holiday,
                    'office_status', phd.office_status,
                    'custom_working_hours', phd.custom_working_hours
                )
                FROM public_holiday_details phd
                WHERE phd.event_id = e.id
            )
            WHEN 'note' THEN (
                SELECT jsonb_build_object(
                    'note_type', nd.note_type,
                    'assigned_staff', nd.assigned_staff,
                    'is_private', nd.is_private,
                    'category', nd.category,
                    'priority', nd.priority,
                    'related_entity_type', nd.related_entity_type,
                    'related_entity_id', nd.related_entity_id,
                    'status', nd.status,
                    'completion_date', nd.completion_date,
                    'completed_by', nd.completed_by
                )
                FROM note_details nd
                WHERE nd.event_id = e.id
            )
        END AS details
    FROM events e
    WHERE e.created_by = $1
        AND ($2::date IS NULL OR e.date >= $2)
        AND ($3::date IS NULL OR e.date <= $3)
        ORDER BY e.date, e.start_time
    "#;

        let query_builder = sqlx::query(&query)
            .bind(user_id)
            .bind(start_date)
            .bind(end_date);

        let rows = query_builder
            .fetch_all(pool)
            .await
            .map_err(CustomErrorsDiary::from)?;

        let result = rows
            .into_iter()
            .map(|row| {
                let event = Event {
                    id: row.get("id"),
                    external_id: row.get("external_id"),
                    event_type: row.get("event_type"),
                    date: row.get("date"),
                    start_time: row.get("start_time"),
                    end_time: row.get("end_time"),
                    title: row.get("title"),
                    description: row.get("description"),
                    created_by: row.get("created_by"),
                    created_at: Some(row.get::<DateTime<Utc>, _>("created_at")),
                    updated_at: Some(row.get::<DateTime<Utc>, _>("updated_at")),
                };
                let details_json: JsonValue = row.get("details");
                let details = match event.event_type {
                    EventType::Viewing => serde_json::from_value(details_json)
                        .map(EventDetails::Viewing)
                        .map_err(CustomErrorsDiary::from),
                    EventType::Appointment => serde_json::from_value(details_json)
                        .map(EventDetails::Appointment)
                        .map_err(CustomErrorsDiary::from),
                    EventType::Inspection => serde_json::from_value(details_json)
                        .map(EventDetails::Inspection)
                        .map_err(CustomErrorsDiary::from),
                    EventType::Note => serde_json::from_value(details_json)
                        .map(EventDetails::Note)
                        .map_err(CustomErrorsDiary::from),
                    EventType::SickLeave => serde_json::from_value(details_json)
                        .map(EventDetails::SickLeave)
                        .map_err(CustomErrorsDiary::from),
                    EventType::StaffMeeting => serde_json::from_value(details_json)
                        .map(EventDetails::StaffMeeting)
                        .map_err(CustomErrorsDiary::from),
                    EventType::Valuation => serde_json::from_value(details_json)
                        .map(EventDetails::Valuation)
                        .map_err(CustomErrorsDiary::from),
                    EventType::Callback => serde_json::from_value(details_json)
                        .map(EventDetails::Callback)
                        .map_err(CustomErrorsDiary::from),
                    EventType::Maintenance => serde_json::from_value(details_json)
                        .map(EventDetails::Maintenance)
                        .map_err(CustomErrorsDiary::from),
                    EventType::PublicHoliday => serde_json::from_value(details_json)
                        .map(EventDetails::PublicHoliday)
                        .map_err(CustomErrorsDiary::from),
                    EventType::StaffHoliday => serde_json::from_value(details_json)
                        .map(EventDetails::StaffHoliday)
                        .map_err(CustomErrorsDiary::from),
                    EventType::Training => serde_json::from_value(details_json)
                        .map(EventDetails::Training)
                        .map_err(CustomErrorsDiary::from),
                }?;
                Ok((event, details))
            })
            .collect::<Result<Vec<_>, CustomErrorsDiary>>()?;
        Ok(result)
    }

    pub async fn update_event(
        &self,
        state: Arc<AppState>,
        event_id: Uuid,
        updated_event: Event,
        updated_details: EventDetails,
    ) -> Result<Event, CustomErrors> {
        let pool = &state.db;
        let mut tx = pool
            .begin()
            .await
            .map_err(|e| CustomErrors::DatabaseError(e.to_string()))?;

        // Update main event
        let event = sqlx::query_as::<_, Event>(
            "UPDATE events SET
    external_id = $1,
    event_type = $2,
    date = $3,
    start_time = $4,
    end_time = $5,
    title = $6,
    description = $7,
    updated_at = CURRENT_TIMESTAMP
    WHERE id = $8
    RETURNING *",
        )
        .bind(&updated_event.external_id)
        .bind(&updated_event.event_type)
        .bind(&updated_event.date)
        .bind(&updated_event.start_time)
        .bind(&updated_event.end_time)
        .bind(&updated_event.title)
        .bind(&updated_event.description)
        .bind(event_id)
        .fetch_optional(&mut *tx)
        .await
        .map_err(|e| CustomErrors::DatabaseError(e.to_string()))?
        .ok_or(CustomErrors::NotFound)?;

        // Delete existing details
        sqlx::query("DELETE FROM viewing_details WHERE event_id = $1")
            .bind(event_id)
            .execute(&mut *tx)
            .await
            .ok();
        sqlx::query("DELETE FROM appointment_details WHERE event_id = $1")
            .bind(event_id)
            .execute(&mut *tx)
            .await
            .ok();
        sqlx::query("DELETE FROM inspection_details WHERE event_id = $1")
            .bind(event_id)
            .execute(&mut *tx)
            .await
            .ok();
        sqlx::query("DELETE FROM leave_details WHERE event_id = $1")
            .bind(event_id)
            .execute(&mut *tx)
            .await
            .ok();
        sqlx::query("DELETE FROM meeting_details WHERE event_id = $1")
            .bind(event_id)
            .execute(&mut *tx)
            .await
            .ok();
        sqlx::query("DELETE FROM valuation_details WHERE event_id = $1")
            .bind(event_id)
            .execute(&mut *tx)
            .await
            .ok();
        sqlx::query("DELETE FROM callback_details WHERE event_id = $1")
            .bind(event_id)
            .execute(&mut *tx)
            .await
            .ok();
        sqlx::query("DELETE FROM maintenance_details WHERE event_id = $1")
            .bind(event_id)
            .execute(&mut *tx)
            .await
            .ok();

        // Insert new details based on type
        match updated_details {
            EventDetails::Viewing(details) => {
                sqlx::query(
    "INSERT INTO viewing_details (event_id, property_id, client_name, contact_number, viewing_type, notification_length)
    VALUES ($1, $2, $3, $4, $5, $6)"
    )
    .bind(event_id)
    .bind(&details.property_id)
    .bind(&details.client_name)
    .bind(&details.contact_number)
    .bind(&details.viewing_type)
    .bind(&details.notification_length)
    .execute(&mut *tx)
    .await
    .map_err(|e| CustomErrors::DatabaseError(e.to_string()))?;
            }
            EventDetails::Appointment(details) => {
                sqlx::query(
    "INSERT INTO appointment_details (event_id, location, property_id, is_private, notification, is_recurring, recurrence_pattern)
    VALUES ($1, $2, $3, $4, $5, $6, $7)"
    )
    .bind(event_id)
    .bind(&details.location)
    .bind(&details.property_id)
    .bind(&details.is_private)
    .bind(&details.notification)
    .bind(&details.is_recurring)
    .bind(&details.recurrence_pattern)
    .execute(&mut *tx)
    .await
    .map_err(|e| CustomErrors::DatabaseError(e.to_string()))?;
            }
            // ... Similar patterns for other event types ...
            _ => return Err(CustomErrors::BadRequest("Invalid event type".to_string())),
        }

        tx.commit()
            .await
            .map_err(|e| CustomErrors::DatabaseError(e.to_string()))?;
        Ok(event)
    }

    pub async fn delete_event(
        &self,
        state: Arc<AppState>,
        event_id: Uuid,
    ) -> Result<(), CustomErrors> {
        let pool = &state.db;
        let mut tx = pool
            .begin()
            .await
            .map_err(|e| CustomErrors::DatabaseError(e.to_string()))?;

        // Delete all related details first
        sqlx::query("DELETE FROM viewing_details WHERE event_id = $1")
            .bind(event_id)
            .execute(&mut *tx)
            .await
            .ok();
        sqlx::query("DELETE FROM appointment_details WHERE event_id = $1")
            .bind(event_id)
            .execute(&mut *tx)
            .await
            .ok();
        sqlx::query("DELETE FROM inspection_details WHERE event_id = $1")
            .bind(event_id)
            .execute(&mut *tx)
            .await
            .ok();
        sqlx::query("DELETE FROM leave_details WHERE event_id = $1")
            .bind(event_id)
            .execute(&mut *tx)
            .await
            .ok();
        sqlx::query("DELETE FROM meeting_details WHERE event_id = $1")
            .bind(event_id)
            .execute(&mut *tx)
            .await
            .ok();
        sqlx::query("DELETE FROM valuation_details WHERE event_id = $1")
            .bind(event_id)
            .execute(&mut *tx)
            .await
            .ok();
        sqlx::query("DELETE FROM callback_details WHERE event_id = $1")
            .bind(event_id)
            .execute(&mut *tx)
            .await
            .ok();
        sqlx::query("DELETE FROM maintenance_details WHERE event_id = $1")
            .bind(event_id)
            .execute(&mut *tx)
            .await
            .ok();

        // Delete the main event
        let result = sqlx::query("DELETE FROM events WHERE id = $1")
            .bind(event_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| CustomErrors::DatabaseError(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(CustomErrors::NotFound);
        }

        tx.commit()
            .await
            .map_err(|e| CustomErrors::DatabaseError(e.to_string()))?;
        Ok(())
    }

    pub async fn get_events_by_date_range(
        &self,
        state: Arc<AppState>,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<Vec<Event>, CustomErrors> {
        let pool = &state.db;
        sqlx::query_as::<_, Event>(
            "SELECT * FROM events
    WHERE date >= $1 AND date <= $2
    ORDER BY date, start_time",
        )
        .bind(start_date)
        .bind(end_date)
        .fetch_all(pool)
        .await
        .map_err(|e| CustomErrors::DatabaseError(e.to_string()))
    }

    pub async fn get_events_by_type(
        &self,
        state: Arc<AppState>,
        event_type: String,
    ) -> Result<Vec<Event>, CustomErrors> {
        let pool = &state.db;
        sqlx::query_as::<_, Event>(
            "SELECT * FROM events
    WHERE event_type = $1::event_type
    ORDER BY date, start_time",
        )
        .bind(event_type)
        .fetch_all(pool)
        .await
        .map_err(|e| CustomErrors::DatabaseError(e.to_string()))
    }
}
