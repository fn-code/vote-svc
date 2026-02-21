use serde::{Serialize};
use crate::app::HttpBaseResponse;

pub fn success<T: Serialize>(data: Option<T>, message: String) -> HttpBaseResponse<T> {
    HttpBaseResponse {
        status: true,
        message,
        error_code: "".to_string(),
        data
    }
}

pub fn error<T: Serialize>(data: Option<T>, message: String, err_code: String) -> HttpBaseResponse<T> {
    HttpBaseResponse {
        status: false,
        message,
        error_code: err_code,
        data
    }
}