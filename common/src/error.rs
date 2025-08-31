use crate::response::Response;
use axum::http::StatusCode;
use axum::response::IntoResponse;

pub type ErrorTuple = (u32, &'static str);

// 错误常量
pub const DB_ERR: (u32, &'static str) = (10001, "数据库错误");
pub const ERR_SYSTEM_ERROR: (u32, &'static str) = (99999, "系统错误");

// 统一应用错误类型
#[derive(Debug)]
pub struct AppError {
    pub code: u32,
    pub message: &'static str,
}

impl AppError {
    pub fn system() -> Self {
        let (code, message) = ERR_SYSTEM_ERROR;
        Self { code, message }
    }
}

// 将业务错误码元组转换为 AppError，into()
impl From<ErrorTuple> for AppError {
    fn from((code, message): ErrorTuple) -> Self {
        Self { code, message }
    }
}

// 将 anyhow::Error 统一映射为系统错误
impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        tracing::error!("system error: {}", err);
        AppError::system()
    }
}

// 原错误转换
impl From<(StatusCode, &'static str)> for AppError {
    fn from((code, msg): (StatusCode, &'static str)) -> Self {
        tracing::error!("system error,{} {}", code, msg);
        AppError::system()
    }
}

// 让错误自动转为统一响应体
impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        // 可按需将 self.code 映射到不同 HTTP 状态码
        let body = Response::<()>::build_failure((self.code, self.message));
        body.into_response()
    }
}
