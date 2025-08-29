pub mod response;

use crate::response::{ErrorEnum, ResponseError};
use axum::http::StatusCode;
use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct AppState {
    db: Option<DatabaseConnection>,
}
impl AppState {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db: Some(db) }
    }

    pub fn new_empty() -> Self {
        Self { db: None }
    }

    pub fn db(&self) -> Result<&DatabaseConnection, (StatusCode, &'static str)> {
        if self.db.is_some() {
            Ok(self.db.as_ref().unwrap())
        } else {
            Err((StatusCode::INTERNAL_SERVER_ERROR, ErrorEnum::DBError.message()))
        }
    }
}
