use axum::Router;
use common::AppState;
use sea_orm::{Database};
use std::env;
use tower_cookies::CookieManagerLayer;
use tower_http::trace::TraceLayer;

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    //加载配置文件
    dotenv::dotenv().expect(".env文件加载失败");

    // 日志
    init_log();

    //提取变量
    let db_url = env::var("DATABASE_URL").unwrap();
    let host = env::var("HOST").expect(".env文件中未配置 HOST 变量");
    let port = env::var("PORT").expect(".env文件中未配置 HOST 变量");
    //拼接监听url
    let server_url = format!("{host}:{port}");
    //获取db实例
    let db = Database::connect(db_url).await?;

    //创建状态变量s
    let shared_state = AppState { db };

    //构建路由 ,注入Cookie层和状态对象
    let app = Router::new()
        .layer(TraceLayer::new_for_http())
        .layer(CookieManagerLayer::new())
        .merge(handlers::routers::routers(shared_state));

    //开启监听
    tracing::info!("server start on {}", server_url);
    let listener = tokio::net::TcpListener::bind(&server_url).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

fn init_log() {
    std::env::set_var("RUST_LOG", "info,tower_http=debug");

    tracing_subscriber::fmt()
        .with_target(false) // 不显示目标模块名
        .with_level(true) // 显示日志级别
        .with_thread_ids(true) // 显示线程ID
        // .with_file(true)     // 显示文件名
        // .with_line_number(true)  // 显示行号
        .init();
}
