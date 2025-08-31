use api::account::info::{AccountInfoDTO, CreateAccountDTO, ModifyAccountDTO, PageQueryDTO};
use axum::extract::{Path, Query, State};
use axum::Json;
use common::error::AppError;
use common::response::{PageResult, Response};
use common::AppState;
use service::account::account_service;

#[utoipa::path(
    get,
    path = "/account/info",
    tag = api::ACCOUNT_TAG,
    params(
        ("page" = u64, Query, description = "页码"),
        ("size" = u64, Query, description = "每页大小")
    ),
)]
pub async fn info(
    State(state): State<AppState>,
    Query(page_query): Query<PageQueryDTO>,
) -> Result<Response<PageResult<AccountInfoDTO>>, AppError> {
    let body = account_service::account_info(state.db()?, &page_query).await?;
    Response::build_success(body)
}

#[utoipa::path(
    post,
    path = "/account/create",
    tag = api::ACCOUNT_TAG,
)]
pub async fn create(
    State(state): State<AppState>,
    Json(create): Json<CreateAccountDTO>,
) -> Result<Response<()>, AppError> {
    account_service::create_account(state.db()?, &create).await?;
    Response::empty_success()
}

#[utoipa::path(
    post,
    path = "/account/modify",
    tag = api::ACCOUNT_TAG,
)]
pub async fn modify(
    State(state): State<AppState>,
    Json(modify): Json<ModifyAccountDTO>,
) -> Result<Response<()>, AppError> {
    account_service::modify_account(state.db()?, &modify).await?;
    Response::empty_success()
}

#[utoipa::path(
    delete,
    path = "/account/delete/{id:u64}",
    tag = api::ACCOUNT_TAG,
)]
pub async fn delete(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Response<()>, AppError> {
    account_service::delete_account(state.db()?, id).await?;
    Response::empty_success()
}
