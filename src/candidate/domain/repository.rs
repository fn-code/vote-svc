use crate::candidate::domain::entities::{CandidateListPage, CandidateFilter};
use crate::candidate::domain::errors::CandidateError;
use async_trait::async_trait;
use mockall::automock;

#[automock]
#[async_trait]
pub trait Repository: Send + Sync {
    async fn find_all(&self, params :CandidateFilter) -> Result<CandidateListPage, CandidateError>;
}