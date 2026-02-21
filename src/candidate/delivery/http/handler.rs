use actix_web::web;
use crate::candidate::delivery::http::get_candidate::get_candidate;


pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/candidates")
            .route("", web::get().to(get_candidate))
    );
}

