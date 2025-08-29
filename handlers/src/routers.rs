use crate::account;
use axum::Router;
use common::AppState;
use utoipa_axum::router::OpenApiRouter;

pub fn routers(app_state: AppState) -> Router {
    let mut router = api::swagger_routers();

    // utoipa-axum宏注册路由
    router = router.merge(account::routes().with_state(app_state));
    // 注册swagger
    router = OpenApiRouter::from(api::swagger_router(router));

    Router::new().merge(router)
}
