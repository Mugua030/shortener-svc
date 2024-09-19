use axum::http::StatusCode;
use axum::response::Json;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize)]
struct ErrorOutput {
    pub error: String,
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error("sql error: {0}")]
    SqlxError(#[from] sqlx::Error),
    #[error("params {0} error")]
    ParamErr(String),
    #[error("Retry limit reached")]
    RetryLimitReached,
}

impl ErrorOutput {
    fn new(error: impl Into<String>) -> Self {
        Self {
            error: error.into(),
        }
    }
}


impl IntoResponse for AppError {
    fn into_response(self) -> Response<axum::body::Body> {
       let status = match &self {
            Self::SqlxError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::ParamErr(_) => StatusCode::BAD_REQUEST,
            Self::RetryLimitReached => StatusCode::FORBIDDEN,
       };

       (status, Json(ErrorOutput::new(self.to_string()))).into_response()
    }
}

