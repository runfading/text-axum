use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(tags(
        (name = super::ACCOUNT_TAG, description = "Customer API endpoints"),
))]
pub struct ApiDoc;

#[utoipa::path(
        method(get, head), // 方法
        path = "/api/health", // 路径
        responses(
                // 响应状态、描述、返回内容、类型
        (status = OK, description = "Success", body = str, content_type = "text/plain")
        )
)]
pub async fn health() -> &'static str {
    "ok"
}

pub mod inner {
    pub mod secret_handlers {
        /// swagger内部处理器
        #[utoipa::path(get, path = "/api/inner/secret", responses((status = OK, body = str)))]
        pub async fn get_secret() -> &'static str {
            "secret"
        }

        /// swagger内部处理器
        #[utoipa::path(
                        post,
                        path = "/api/inner/secret",
                        responses((status = OK, description = "OK"))
                )]
        pub async fn post_secret() {
            println!("You posted a secret")
        }
    }
}
