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
        // 这个宏不能同时注册相同请求类型的接口，即使路径不同
        .routes(routes!(controller::modify))
}
