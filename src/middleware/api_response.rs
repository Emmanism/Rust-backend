use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use crate::types::ApiResponse; 


/* pub fn success<T: Serialize>(
    status: StatusCode,
    message: &str,
    data: Option<T>,
) -> (StatusCode, Json<ApiResponse<T>>) {
    (
        status,
        Json(ApiResponse {
            success: true,
            message: message.to_string(),
            data,
        }),
    )
} */

pub fn success<T: Serialize>(
    status: StatusCode,
    message: &str,
    data: Option<T>,
) -> Response {
    (
        status,
        Json(ApiResponse {
            success: true,
            message: message.to_string(),
            data,
        }),
    )
        .into_response()
}

/* pub fn error(
    status: StatusCode,
    message: &str,
) -> (StatusCode, Json<ApiResponse<()>>) {
    (
        status,
        Json(ApiResponse {
            success: false,
            message: message.to_string(),
            data: None,
        }),
    )
} */

pub fn error(
    status: StatusCode,
    message: &str,
) -> Response {
    (
        status,
        Json(ApiResponse::<()> {
            success: false,
            message: message.to_string(),
            data: None,
        }),
    )
        .into_response()
}