use axum::Router;
use common::AppState;
use sea_orm::Database;
use std::env;
use tower_cookies::CookieManagerLayer;
use tower_http::trace;
use tower_http::trace::TraceLayer;
use tracing::Level;
use tracing_subscriber::filter::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{self, prelude::*};

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    //加载配置文件
    dotenv::dotenv().expect(".env文件加载失败");

    // 日志
    // init_log();

    setup_tracing();
    //提取变量
    let db_url = env::var("DATABASE_URL").expect(".env文件中未配置 DATABASE_URL 变量");
    let host = env::var("HOST").expect(".env文件中未配置 HOST 变量");
    let port = env::var("PORT").expect(".env文件中未配置 HOST 变量");
    //拼接监听url
    let server_url = format!("{host}:{port}");
    //获取db实例
    let db = Database::connect(db_url).await?;

    //创建状态变量s
    let shared_state = AppState::new(db);

    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
        .on_request(trace::DefaultOnRequest::new().level(Level::INFO))
        .on_response(trace::DefaultOnResponse::new().level(Level::INFO));

    //构建路由 ,注入Cookie层和状态对象
    let app = Router::new()
        .layer(CookieManagerLayer::new())
        .merge(handlers::routers::routers(shared_state))
        .layer(trace_layer); // 和顺序还有一定的关系，要在路由下面

    //开启监听
    tracing::info!("server start on {}", server_url);
    let listener = tokio::net::TcpListener::bind(&server_url).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

fn setup_tracing() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")); // 默认 info

    tracing_subscriber::registry()
        .with(filter)
        .with(
            tracing_subscriber::fmt::layer()
                .with_target(true)
                .with_level(true)
                .with_thread_ids(true),
        )
        .init();

    tracing::debug!(
        "tracing 初始化完成，日志级别: {}",
        env::var("RUST_LOG").unwrap_or("未设置".into())
    );
}
