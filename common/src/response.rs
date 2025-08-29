use axum::http::StatusCode;
use axum::response::{IntoResponse, Response as AxumResponse};
use axum::Json;
use serde::Serialize;
use std::fmt;

#[derive(Serialize)]
pub struct Response<T: serde::Serialize> {
    code: i32,
    message: String,
    data: Option<T>,
}

impl<T: serde::Serialize> Response<T> {
    pub fn build_success(data: T) -> Self {
        Response {
            code: 200,
            message: "success".to_string(),
            data: Some(data),
        }
    }

    pub fn build_failure<Err>(error: Err) -> Response<T>
    where
        Err: ResponseError,
    {
        Response {
            code: error.code(),
            message: error.message().to_string(),
            data: None,
        }
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

pub trait ResponseError {
    fn code(&self) -> i32;
    fn message(&self) -> &str;
}

#[derive(Debug, Clone, Serialize)]
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

#[derive(Debug, Clone)]
pub enum ErrorEnum {
    SysError(String),
}

impl ResponseError for ErrorEnum {
    fn code(&self) -> i32 {
        match self {
            ErrorEnum::SysError(_) => 9999,
        }
    }

    fn message(&self) -> &str {
        match self {
            ErrorEnum::SysError(msg) => msg,
        }
    }
}

// 实现 Display trait 以便更好的错误显示
impl fmt::Display for ErrorEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message())
    }
}

// 实现 std::error::Error trait（可选，但推荐）
impl std::error::Error for ErrorEnum {}
