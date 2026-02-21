use crate::candidate::domain;
use crate::candidate::domain::CandidateError;
use crate::candidate::domain::Repository;
use crate::infrastructure::database::postgres::Postgres;
use anyhow::anyhow;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::candidate::repository::model::Candidate;

pub struct PostgresRepo {
    postgres: sqlx::PgPool,
}
impl PostgresRepo {
    pub async fn new(postgres: Arc<Mutex<Postgres>>) -> anyhow::Result<Self> {
        let guard = postgres.lock().await;
        let pool = guard
            .pool()
            .ok_or_else(|| anyhow!("DB pool is not initialized"))?;
        Ok(PostgresRepo { postgres: pool })
    }

    fn transform_candidate(&self, candidates: Vec<Candidate>) -> Vec<domain::Candidate> {
        candidates
            .into_iter() 
            .map(|c| domain::Candidate {
                id: c.id.to_string(),
                vote_number: c.vote_number,
                president_name: c.president_name,
                vice_president_name: c.vice_president_name,
                president_nim: c.president_nim,
                vice_president_nim: c.vice_president_nim,
                president_photo: c.president_photo,
                vice_president_photo: c.vice_president_photo,
                status: c.status,
                created_by: c.created_by,
                created_at: c.created_at,
                updated_at: c.updated_at,
            })
            .collect()
    }
}

#[async_trait]
impl Repository for PostgresRepo {
    async fn find_all(&self) -> Result<Vec<domain::Candidate>, CandidateError> {
        let candidates_result = sqlx::query_as::<_, Candidate>(
            "SELECT id
            , vote_number
            , president_name
            , vice_president_name
            , president_nim
            , vice_president_nim
            , president_photo
            , vice_president_photo
            , status
            , created_by
            , created_at
            , updated_at
            FROM candidates ",
        )
        .fetch_all(&self.postgres);

        let candidates_model = match candidates_result.await {
            Ok(result) => result,
            Err(e) => match e {
                sqlx::Error::RowNotFound => return Err(CandidateError::NotFound(e.to_string())),
                _ => return Err(CandidateError::UnknownError(e.to_string())),
            }
        };

        let candidates_result = self.transform_candidate(candidates_model);

        Ok(candidates_result)
    }

}
