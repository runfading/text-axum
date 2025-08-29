pub mod controller;

use common::AppState;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(
            controller::info,
            controller::create,
            controller::delete
        ))
        .routes(routes!(controller::modify))
}
