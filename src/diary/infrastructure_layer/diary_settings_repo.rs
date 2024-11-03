use crate::{diary::domain_layer::diary_settings::DiarySettings, AppState};
use actix_web::error::ResponseError;
use chrono::Utc;
use derive_more::Display;
use serde::Serialize;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Display, Serialize)]
pub enum CustomErrors {
    #[display(fmt = "Database error: {}", _0)]
    DatabaseError(String),
    #[display(fmt = "Not authorized")]
    NotAuthorized,
}

impl ResponseError for CustomErrors {}

pub struct DiarySettingsRepository {}

impl DiarySettingsRepository {
    pub fn new() -> Self {
        DiarySettingsRepository {}
    }

    pub async fn get_all_diary_settings(
        &self,
        state: Arc<AppState>,
    ) -> Result<Vec<DiarySettings>, CustomErrors> {
        let records = sqlx::query_as::<_, DiarySettings>("SELECT * FROM diary_settings")
            .fetch_all(&state.db)
            .await;

        match records {
            Ok(settings) => Ok(settings),
            Err(e) => Err(CustomErrors::DatabaseError(e.to_string())),
        }
    }
    pub async fn create_diary_settings(
        &self,
        state: Arc<AppState>,
        mut new_settings: DiarySettings,
    ) -> Result<DiarySettings, CustomErrors> {
        // Generate a new UUID for diary_id if it's not provided
        let uuid = new_settings.diary_id.unwrap_or_else(Uuid::new_v4);

        // Set a default color if diary_colour is not provided
        if new_settings.diary_colour.is_none() {
            new_settings.diary_colour = Some("#33B3F0".to_string());
        }

        // Create the new settings record in the database
        let record = sqlx::query_as::<_, DiarySettings>(
            "INSERT INTO diary_settings (diary_id, staff_id, diary_colour, popup_notifi_en, email_notifi_en, updated_at) 
             VALUES ($1, $2, $3, $4, $5, $6) 
             RETURNING *"
        )
        .bind(uuid)
        .bind(new_settings.staff_id)
        .bind(new_settings.diary_colour)
        .bind(new_settings.popup_notifi_en)
        .bind(new_settings.email_notifi_en)
        .bind(Some(Utc::now())) // Set updated_at to the current time
        .fetch_one(&state.db)
        .await;

        // Match on the result and return appropriately
        match record {
            Ok(settings) => Ok(settings),
            Err(e) => Err(CustomErrors::DatabaseError(e.to_string())),
        }
    }

    pub async fn update_diary_settings(
        &self,
        state: Arc<AppState>,
        diary_id: Uuid,
        updated_settings: DiarySettings,
    ) -> Result<DiarySettings, CustomErrors> {
        let record = sqlx::query_as::<_, DiarySettings>(
            "UPDATE diary_settings SET staff_id = $1, diary_colour = $2, popup_notifi_en = $3, email_notifi_en = $4, updated_at = $5 WHERE diary_id = $6 RETURNING *"
        )
        .bind(updated_settings.staff_id)
        .bind(updated_settings.diary_colour)
        .bind(updated_settings.popup_notifi_en)
        .bind(updated_settings.email_notifi_en)
        .bind(updated_settings.updated_at)
        .bind(diary_id)
        .fetch_one(&state.db)
        .await;

        match record {
            Ok(settings) => Ok(settings),
            Err(e) => Err(CustomErrors::DatabaseError(e.to_string())),
        }
    }

    pub async fn delete_diary_settings(
        &self,
        state: Arc<AppState>,
        diary_id: Uuid,
    ) -> Result<(), CustomErrors> {
        let result = sqlx::query("DELETE FROM diary_settings WHERE diary_id = $1")
            .bind(diary_id)
            .execute(&state.db)
            .await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(CustomErrors::DatabaseError(e.to_string())),
        }
    }

    pub async fn get_diary_settings_by_id(
        &self,
        state: Arc<AppState>,
        diary_id: Uuid,
    ) -> Result<DiarySettings, CustomErrors> {
        let record =
            sqlx::query_as::<_, DiarySettings>("SELECT * FROM diary_settings WHERE staff_id = $1")
                .bind(diary_id)
                .fetch_one(&state.db)
                .await;

        match record {
            Ok(settings) => Ok(settings),
            Err(e) => Err(CustomErrors::DatabaseError(e.to_string())),
        }
    }
}
