use crate::candidate::usecase::get::*;
use crate::utils::{app, response};
use actix_web::{HttpResponse, web};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CandidateListQuery {
    id: Option<String>,
    page: Option<u32>,
    limit: Option<u32>,
}

pub async fn get_candidate(handler: web::Data<app::AppHandlerData>, q: web::Query<CandidateListQuery>) -> HttpResponse {

    let request = Request {
        id: q.id.clone(),
        page: Some(q.page.unwrap_or(1)),
        limit: Some(q.limit.unwrap_or(10)),
    };

    println!("-> Received request: {:?}", request);
    
    let candidate_uc = handler.candidate_uc.get.handle(request).await;

    let response = match candidate_uc {
        Ok(response) => response,
        Err(e) => {
            println!("Error: {}", e);
            return HttpResponse::InternalServerError().json(response::error::<()>(
                None,
                "failed process data".into(),
                "-1".into(),
            ));
        }
    };

    HttpResponse::Ok().json(response::success(
        Some(response),
        "Successfully processed candidate".into(),
    ))
}

// rust
#[cfg(test)]
mod tests {
    use super::get_candidate;
    use actix_web::{App, http::StatusCode, test, web};
    use async_trait::async_trait;
    use serde_json::Value;
    use std::sync::Arc;
    use crate::candidate;
    use crate::candidate::domain::CandidateError;
    use crate::candidate::usecase::get::{Interactor, Request, Response};
    use crate::utils::app;

    // Helper to create web::Data with a custom MockGet instance
    fn init_app_data(get_impl: Arc<dyn Interactor>) -> web::Data<app::AppHandlerData> {

        let candidate_uc = candidate::usecase::UseCase {
            get: get_impl,
        };

        web::Data::new(app::AppHandlerData { candidate_uc })
    }

    #[actix_rt::test]
    async fn test_get_candidate_success() {

        #[derive(Clone, Debug)]
        struct MockGetSuccess;

        #[async_trait]
        impl Interactor for MockGetSuccess {
            async fn handle(&self, _: Request) -> Result<Response, CandidateError> {

                let candidates = vec![
                    candidate::domain::Candidate {
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
                    },
                ];

                Ok(
                    Response {
                        candidates: candidates,
                        total: 10,
                        limit:10,
                        page:1,
                    }
                )
            }
        }

        let get_impl: Arc<dyn Interactor> = Arc::new(MockGetSuccess);

        let app_data = init_app_data(get_impl);

        let app = test::init_service(
            App::new()
                .app_data(app_data.clone())
                .route("/candidates", web::get().to(get_candidate)),
        )
        .await;

        // Act
        let req = test::TestRequest::get().uri("/candidates").to_request();
        let resp = test::call_service(&app, req).await;

        // Assert
        assert_eq!(resp.status(), StatusCode::OK);
        let body: Value = test::read_body_json(resp).await;

        // on success handler wraps the usecase response inside utils::response::success
        // adjust assertions according to your utils::response format.
        assert!(
            body.get("data").is_some(),
            "expected `data` key in response"
        );
    }


    #[actix_rt::test]
    async fn test_get_candidate_failure() {
        #[derive(Clone, Debug)]
        struct MockGetFailure;

        #[async_trait]
        impl Interactor for MockGetFailure {
            async fn handle(&self, _: Request) -> Result<Response, CandidateError> {
                Err(CandidateError::NotFound("Candidates not found".into()))
            }
        }


        let get_impl: Arc<dyn Interactor> = Arc::new(MockGetFailure);

        let app_data = init_app_data(get_impl);

        let app = test::init_service(
            App::new()
                .app_data(app_data.clone())
                .route("/candidates", web::get().to(get_candidate)),
        )
        .await;

        // Act
        let req = test::TestRequest::get().uri("/candidates").to_request();
        let resp = test::call_service(&app, req).await;

        // Assert
        assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
        let body: Value = test::read_body_json(resp).await;

        assert_eq!(body.get("status").unwrap(), false, "expected status to be false in error response");
        assert!(
            body.get("error_code").is_some() || body.get("message").is_some(),
            "expected error_code and message is not empty in error response"
        );
    }
}
