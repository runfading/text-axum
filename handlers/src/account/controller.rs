use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Json;
use common::account::info::{AccountInfoDTO, CreateAccountDTO, ModifyAccountDTO, PageQueryDTO};
use common::response::ErrorEnum;
use common::response::{PageResult, Response};
use common::AppState;
use service::account::account_service;

pub async fn info(
    State(state): State<AppState>,
    Query(page_query): Query<PageQueryDTO>,
) -> Result<Response<PageResult<AccountInfoDTO>>, (StatusCode, &'static str)> {
    let body = match account_service::account_info(&state.db, &page_query).await {
        Ok(accounts) => Response::build_success(accounts),
        Err(err) => Response::build_failure(ErrorEnum::SysError(err.to_string())),
    };
    Ok(body)
}

pub async fn create(
    State(state): State<AppState>,
    Json(create): Json<CreateAccountDTO>,
) -> Result<Response<()>, (StatusCode, &'static str)> {
    let result = match account_service::create_account(&state.db, &create).await {
        Ok(_) => Response::build_success(()),
        Err(err) => Response::build_failure(ErrorEnum::SysError(err.to_string())),
    };

    Ok(result)
}

pub async fn modify(
    State(state): State<AppState>,
    Json(modify): Json<ModifyAccountDTO>,
) -> Result<Response<()>, (StatusCode, &'static str)> {
    let result = match account_service::modify_account(&state.db, &modify).await {
        Ok(_) => Response::build_success(()),
        Err(err) => Response::build_failure(ErrorEnum::SysError(err.to_string())),
    };

    Ok(result)
}

pub async fn delete(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Response<()>, (StatusCode, &'static str)> {
    let result = match account_service::delete_account(&state.db, id).await {
        Ok(_) => Response::build_success(()),
        Err(err) => Response::build_failure(ErrorEnum::SysError(err.to_string())),
    };

    Ok(result)
}
