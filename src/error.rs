use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

#[derive(Debug, thiserror::Error)]
pub enum AppError
{
    #[error("bad request")]
    BadRequest,

    #[error("internal error")]
    Internal,
}

impl IntoResponse for AppError
{
    fn into_response(self) -> Response
    {
        let (status, msg) = match self
        {
            AppError::BadRequest => (StatusCode::BAD_REQUEST, "bad request"),
            AppError::Internal => (StatusCode::INTERNAL_SERVER_ERROR, "internal error"),
        };

        (status, Json(json!({ "error": msg }))).into_response()
    }
}
