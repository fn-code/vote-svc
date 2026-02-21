use thiserror::Error;


#[derive(Debug, Error)]
#[derive(Clone)]
pub enum CandidateError {
    #[error("CandidateError::NotFound: {0}")]
    NotFound(String),
    #[error("CandidateError::UnknownError: {0}")]
    UnknownError(String),
}

