use axum::{http::StatusCode, Json};
use serde::Serialize;
use crate::types::ApiResponse; 


pub fn success<T: Serialize>(
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
}

pub fn error(
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
}