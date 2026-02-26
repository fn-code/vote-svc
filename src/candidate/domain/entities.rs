use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq)]
pub struct CandidateFilter {
    pub id: Option<String>,
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Candidate {
    pub id: String,
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CandidateListPage {
    pub total: usize,
    pub candidates: Vec<Candidate>,
}