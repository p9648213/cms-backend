use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

pub struct AppError {
    code: StatusCode,
    message: String,
}

#[derive(Serialize, Deserialize)]
struct ErrorRespone {
    error: String,
}

impl AppError {
    pub fn new(code: StatusCode, message: &str) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (
            self.code,
            Json(ErrorRespone {
                error: self.message.clone(),
            }),
        )
            .into_response()
    }
}
