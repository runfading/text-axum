use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AccountInfoDTO {
    pub id: u64,
    pub name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateAccountDTO {
    pub name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModifyAccountDTO {
    pub id: u64,
    pub name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PageQueryDTO {
    pub page: u64,
    pub size: u64,
}
