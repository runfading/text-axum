pub mod response;
pub mod account;

use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}

