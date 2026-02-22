use serde::{Serialize};
use crate::utils::app;


pub fn success<T: Serialize>(data: Option<T>, message: String) -> app::HttpBaseResponse<T> {
    app::HttpBaseResponse {
        status: true,
        message,
        error_code: "".to_string(),
        data
    }
}

pub fn error<T: Serialize>(data: Option<T>, message: String, err_code: String) -> app::HttpBaseResponse<T> {
    app::HttpBaseResponse {
        status: false,
        message,
        error_code: err_code,
        data
    }
}