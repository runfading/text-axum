use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema,Validate)]
pub struct AccountInfoDTO {
    pub id: u64,

    #[validate(length(max=20))]
    pub name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema,Validate)]
pub struct CreateAccountDTO {
    #[validate(length(max=20))]
    pub name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema,Validate)]
pub struct ModifyAccountDTO {
    pub id: u64,
    #[validate(length(max=20))]
    pub name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema,Validate)]
pub struct PageQueryDTO {
    pub page: u64,
    #[validate(range(max=1000))]
    pub size: u64,
}
