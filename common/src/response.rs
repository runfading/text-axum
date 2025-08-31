use crate::error::{AppError, ErrorTuple};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response as AxumResponse};
use axum::Json;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct Response<T: serde::Serialize> {
    code: u32,
    message: &'static str,
    data: Option<T>,
}

impl<T: serde::Serialize> Response<T> {
    pub fn build_success(data: T) -> Result<Self, AppError> {
        Ok(Response {
            code: 200,
            message: "success",
            data: Some(data),
        })
    }

    pub fn build_failure((code, message): ErrorTuple) -> Result<Self, AppError> {
        Ok(Response {
            code,
            message,
            data: None,
        })
    }

    pub fn empty_success() -> Result<Self, AppError> {
        Ok(Response {
            code: 200,
            message: "success",
            data: None,
        })
    }
}

impl<T> IntoResponse for Response<T>
where
    T: serde::Serialize,
{
    fn into_response(self) -> AxumResponse {
        let status = if self.code == 200 {
            StatusCode::OK
        } else {
            StatusCode::BAD_REQUEST // 或根据错误码映射到不同的 HTTP 状态码
        };

        (status, Json(self)).into_response()
    }
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct PageResult<T: Serialize> {
    pub total: u64,
    pub page: u64,
    pub data: Vec<T>,
}

impl<T: Serialize> PageResult<T> {
    pub fn new(total: u64, page: u64, data: Vec<T>) -> Self {
        Self { total, page, data }
    }
}
