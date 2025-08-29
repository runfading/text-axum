use crate::account;
use axum::Router;
use common::AppState;

pub fn routers(app_state: AppState) -> Router {
    Router::new()
        .nest("/account", account::routers())
        .with_state(app_state)
}
