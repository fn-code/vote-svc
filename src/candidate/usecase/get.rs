use crate::candidate::domain::Candidate;
use crate::candidate::domain::CandidateError;
use crate::candidate::domain::Repository;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[async_trait]
pub trait Interactor: Send + Sync {
    async fn handle(&self, _: Request) -> Result<Response, CandidateError>;
}

pub struct GetCandidateUseCase<R: ?Sized + Send + Sync>
where
    R: Repository,
{
    repository: Arc<R>,
}

pub struct Request {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    pub total: i32,
    pub page: i32,
    pub limit: i32,
    pub candidates: Vec<Candidate>,
}

impl<R: ?Sized + Send + Sync> GetCandidateUseCase<R>
where
    R: Repository,
{
    pub fn new(user_repo: Arc<R>) -> Self {
        Self {
            repository: user_repo,
        }
    }
}

#[async_trait]
impl<R: ?Sized + Send + Sync> Interactor for GetCandidateUseCase<R>
where
    R: Repository,
{
    async fn handle(&self, _: Request) -> Result<Response, CandidateError> {
        let candidates = self.repository.find_all().await?;

        Ok(Response {
            candidates: candidates,
            total: 10,
            limit: 10,
            page: 1,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::candidate::domain;
    use crate::candidate::domain::{CandidateError, Repository};
    use crate::candidate::usecase::get;
    use crate::candidate::usecase::get::Interactor;
    use std::sync::Arc;

    // #[tokio::test]
    #[tokio::test(flavor = "current_thread")]
    async fn test_get() {
        struct TestCase
        {
            name: &'static str,
            input_id: String,
            mock_fn: Box<dyn Fn() -> Box<dyn Repository> + Send + Sync + 'static>,
            ret_val: Option<Vec<domain::Candidate>>,
            ret_err: Option<CandidateError>,
        }

        let candidates_ret_ok = vec![domain::Candidate {
            id: "1".to_string(),
            vote_number: 1,
            president_name: "Alice".to_string(),
            vice_president_name: "Bob".to_string(),
            president_nim: "123".to_string(),
            vice_president_nim: "456".to_string(),
            president_photo: "alice.jpg".to_string(),
            vice_president_photo: "bob.jpg".to_string(),
            status: true,
            created_by: "admin".to_string(),
            created_at: chrono::Utc::now(),
            updated_at: Some(chrono::Utc::now()),
        }];


        
        let cases = [
            TestCase {
                name: "success",
                input_id: "1".to_string(),
                mock_fn: Box::new(|| {

                    let mut repo_mock = domain::MockRepository::new();

                    repo_mock.expect_find_all().times(1).returning(|| {
                        let candidates_ret_ok = vec![domain::Candidate {
                            id: "1".to_string(),
                            vote_number: 1,
                            president_name: "Alice".to_string(),
                            vice_president_name: "Bob".to_string(),
                            president_nim: "123".to_string(),
                            vice_president_nim: "456".to_string(),
                            president_photo: "alice.jpg".to_string(),
                            vice_president_photo: "bob.jpg".to_string(),
                            status: true,
                            created_by: "admin".to_string(),
                            created_at: chrono::Utc::now(),
                            updated_at: Some(chrono::Utc::now()),
                        }];

                        Ok(candidates_ret_ok)
                    });

                    Box::new(repo_mock)
                }),
                ret_val: Some(candidates_ret_ok.clone()),
                ret_err: None,
            },
            TestCase {
                name: "failure - repository error",
                input_id: "1".to_string(),
                mock_fn: Box::new(|| {
                    let mut repo_mock = domain::MockRepository::new();
                    repo_mock.expect_find_all().times(1).returning(|| {
                        Err(CandidateError::UnknownError(
                            "Database connection error".to_string(),
                        ))
                    });

                    Box::new(repo_mock)
                }),
                ret_val: None,
                ret_err: Some(CandidateError::UnknownError(
                    "Database connection error".to_string(),
                )),
            },
            TestCase {
                name: "failure - not found error",
                input_id: "1".to_string(),
                mock_fn: Box::new(|| {
                    let mut repo_mock = domain::MockRepository::new();
                    repo_mock.expect_find_all().times(1).returning(|| {
                        Err(CandidateError::NotFound(
                            "candidate not found".to_string(),
                        ))
                    });

                    Box::new(repo_mock)
                }),
                ret_val: None,
                ret_err: Some(CandidateError::NotFound(
                    "candidate not found".to_string(),
                )),
            },
        ];

        for tc in cases.iter() {
            let mock_fn = &tc.mock_fn;
            let repo_box: Box<dyn domain::Repository> = mock_fn();

            let repo_arc: Arc<dyn Repository> = Arc::from(repo_box);

            let get_usecase = get::GetCandidateUseCase::new(repo_arc);

            let result = get_usecase
                .handle(get::Request {
                    id: tc.input_id.clone(),
                })
                .await;

            let result_clone = result.clone();

            println!("Running test case: {}", tc.name);
            match &tc.ret_val {
                Some(ret_val) => {
                    assert!(result.is_ok(), "expected success but got error");
                    let res = result.ok().unwrap();
                    assert_eq!(
                        res.candidates.len(),
                        ret_val.len(),
                        "candidates length mismatch"
                    );
                    assert_eq!(res.candidates[0].id, ret_val[0].id, "candidate id mismatch");
                }
                _ => {}
            }

            match &tc.ret_err {
                Some(expected_err) => {
                    assert!(result_clone.is_err(), "expected error but got success");
                    let err = result_clone.err().unwrap();
                    assert_eq!(
                        format!("{:?}", err),
                        format!("{:?}", expected_err),
                        "error mismatch"
                    );
                }
                _ => {}
            }
        }
    }
}
