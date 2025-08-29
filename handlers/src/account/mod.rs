pub mod controller;

use axum::routing::{delete, get, post};
use axum::Router;
use common::AppState;

pub fn routers() -> Router<AppState> {
    Router::new()
        .route("/info", get(controller::info))
        .route("/create", post(controller::create))
        .route("/modify", post(controller::modify))
        .route("/delete/{id}", delete(controller::delete))
}
