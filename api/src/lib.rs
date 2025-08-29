use crate::swagger::__path_health;
use crate::swagger::{health, inner, ApiDoc};
use axum::Router;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;
use utoipa_swagger_ui::SwaggerUi;

pub mod account;
mod swagger;

// 定义账户相关API的标签常量
pub const ACCOUNT_TAG: &str = "Account";

pub fn swagger_routers() -> OpenApiRouter {
    OpenApiRouter::with_openapi(ApiDoc::openapi())
        .routes(routes!(health))
        .routes(routes!(
            inner::secret_handlers::get_secret,
            inner::secret_handlers::post_secret
        ))
}

pub fn swagger_router(api: OpenApiRouter) -> Router {
    let (router, api) = api.split_for_parts();
    router.merge(SwaggerUi::new("/swagger-ui").url("/apidoc/openapi.json", api))
}
