use crate::candidate::domain::entities::Candidate;
use crate::candidate::domain::errors::CandidateError;
use async_trait::async_trait;
use mockall::automock;

#[automock]
#[async_trait]
pub trait Repository: Send + Sync {
    async fn find_all(&self) -> Result<Vec<Candidate>, CandidateError>;
}