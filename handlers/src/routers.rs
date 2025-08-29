use crate::account;
use axum::Router;
use utoipa_axum::router::OpenApiRouter;
use common::AppState;

pub fn routers(app_state: AppState) -> Router {
    let mut router = api::swagger_routers();
    router = router.merge(account::routes().with_state(app_state));
    router = OpenApiRouter::from(api::swagger_router(router));

    Router::new()
        .merge(router)
}
