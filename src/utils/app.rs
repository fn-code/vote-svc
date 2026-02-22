use serde::{Deserialize, Serialize};
use crate::candidate;

#[derive(Clone)]
pub struct AppHandlerData {
    pub candidate_uc: candidate::usecase::UseCase
}


#[derive(Serialize,Deserialize,Debug)]
pub struct HttpBaseResponse<T> {
    pub status: bool,
    pub message: String,
    pub error_code: String,
    pub data: Option<T>
}