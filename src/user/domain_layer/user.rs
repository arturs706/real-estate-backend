use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{self, FromRow};
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize, FromRow, Debug)]
pub struct StaffUser {
    pub user_id: Option<Uuid>,
    pub name: Option<String>,
    pub username: String,
    pub mob_phone: Option<String>,
    pub passwd: String,
    pub acc_level: Option<UserLevel>,
    pub status: Option<UserStatus>,
    pub a_created: Option<NaiveDateTime>,
}

#[derive(Clone, Serialize, Deserialize, FromRow, Debug)]
pub struct StaffUserFullNames {
    pub user_id: Uuid,
    pub name: String,
}

#[derive(Clone, Serialize, Deserialize, Debug, sqlx::Type)]
#[sqlx(type_name = "user_level", rename_all = "lowercase")]
pub enum UserLevel {
    Admin,
    Manager,
    Staff,
    Trainee,
}

#[derive(Clone, Serialize, Deserialize, Debug, sqlx::Type)]
#[sqlx(type_name = "user_status", rename_all = "lowercase")]
pub enum UserStatus {
    Active,
    Suspended,
}
