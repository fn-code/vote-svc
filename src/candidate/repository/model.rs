use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Candidate {
    pub id: Uuid,
    pub vote_number: i32,
    pub president_name: String,
    pub vice_president_name: String,
    pub president_nim: String,
    pub vice_president_nim: String,
    pub president_photo: String,
    pub vice_president_photo: String,
    pub status: bool,
    pub created_by:String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}