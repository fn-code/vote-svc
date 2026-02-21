use std::sync::Arc;
use crate::candidate::domain::Repository;
use crate::candidate::usecase::get;
use crate::candidate::usecase::get::{GetCandidateUseCase};


#[derive(Clone)]
pub struct UseCase
{
    pub get: Arc<dyn get::Interactor>,
}

impl UseCase {

    pub fn new(candidate_repo: Arc<dyn Repository + Send + Sync>) -> Self {

        
        let get_uc = GetCandidateUseCase::new(candidate_repo);
        let get_uc_arc = Arc::new(get_uc);

        Self { get: get_uc_arc }
    }

}