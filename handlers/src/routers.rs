use crate::account;
use axum::Router;
use common::AppState;

pub fn routers(app_state: AppState) -> Router {
    Router::new()
        .nest("/account", account::routers())
        .with_state(app_state)
        .merge(swagger_routers())
}

pub fn swagger_routers() -> Router {
    api::swagger_routers()
}
