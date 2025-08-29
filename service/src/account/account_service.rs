use common::account::info::{AccountInfoDTO, CreateAccountDTO, ModifyAccountDTO, PageQueryDTO};
use common::response::PageResult;
use db::account::entities::account::ActiveModel;
use db::account::entities::prelude::Account;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Delete, EntityTrait, PaginatorTrait, Update};

pub async fn account_info(
    db: &DatabaseConnection,
    page_query: &PageQueryDTO,
) -> anyhow::Result<PageResult<AccountInfoDTO>> {
    let paginator = Account::find().paginate(db, page_query.size);

    let total = paginator.num_items().await?;

    let users = paginator
        .fetch_page(page_query.page - 1)
        .await?
        .into_iter()
        .map(|x| AccountInfoDTO {
            id: x.id,
            name: x.name,
        })
        .collect::<Vec<AccountInfoDTO>>();

    Ok(PageResult::new(total, page_query.page, users))
}

pub async fn create_account(
    db: &DatabaseConnection,
    account_info: &CreateAccountDTO,
) -> anyhow::Result<()> {
    ActiveModel {
        id: sea_orm::ActiveValue::NotSet,
        name: sea_orm::Set(account_info.name.clone()),
    }
    .save(db)
    .await?;
    Ok(())
}

pub async fn modify_account(
    db: &DatabaseConnection,
    account_info: &ModifyAccountDTO,
) -> anyhow::Result<()> {
    Update::one(ActiveModel {
        id: sea_orm::Set(account_info.id),
        name: sea_orm::Set(account_info.name.clone()),
    })
    .exec(db)
    .await?;
    Ok(())
}

pub async fn delete_account(db: &DatabaseConnection, id: u64) -> anyhow::Result<()> {
    Delete::one(ActiveModel {
        id: sea_orm::Set(id),
        name: sea_orm::NotSet,
    })
    .exec(db)
    .await?;
    Ok(())
}
