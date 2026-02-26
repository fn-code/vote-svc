use crate::candidate::domain;
use crate::candidate::domain::{CandidateError, CandidateFilter};
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
    async fn find_all(&self, params :CandidateFilter) -> Result<domain::CandidateListPage, CandidateError> {

        let mut qb: sqlx::QueryBuilder<sqlx::Postgres> = sqlx::QueryBuilder::new(
            r#"
        SELECT id
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
        FROM candidates
        "#,
        );

        let mut count_qb: sqlx::QueryBuilder<sqlx::Postgres> =
            sqlx::QueryBuilder::new(r#"SELECT COUNT(*)::BIGINT FROM candidates"#);

        // Adds " WHERE " once, and then " AND " between predicates.
        let mut separated = qb.separated(" WHERE ");
        let mut count_separated = count_qb.separated(" WHERE ");


        if let Some(id) = &params.id {
            separated.push("id = ").push_bind(id);
            count_separated.push("id = ").push_bind(id);
        }


        // set default limit and offset
        let mut limit = 10;
        let mut offset = 0;

        if let Some(page) = params.page {
            limit = params.limit.unwrap_or(10);
            offset = (page - 1) * limit;
        }

        qb.push(" LIMIT ").push_bind(limit as i64).push(" OFFSET ").push_bind(offset as i64);

        // Build typed query
        let query = qb.build_query_as::<Candidate>();

        let candidates_result = query.fetch_all(&self.postgres);

        let total: i64 = count_qb
            .build_query_scalar()
            .fetch_one(&self.postgres)
            .await
            .map_err(|e| CandidateError::UnknownError(e.to_string()))?;

        let candidates_model = match candidates_result.await {
            Ok(result) => result,
            Err(e) => match e {
                sqlx::Error::RowNotFound => return Err(CandidateError::NotFound(e.to_string())),
                _ => return Err(CandidateError::UnknownError(e.to_string())),
            }
        };

        let candidates_result = self.transform_candidate(candidates_model);

        Ok(domain::CandidateListPage {
            total: total as usize,
            candidates: candidates_result,
        })
    }

}
