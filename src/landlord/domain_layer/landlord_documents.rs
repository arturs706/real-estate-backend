use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize, FromRow, Debug)]
pub struct LandlordDocs {
    pub landlord_id: Uuid,
    pub id_document: Option<String>,
}
