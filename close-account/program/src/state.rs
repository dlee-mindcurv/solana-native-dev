use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub enum User {
    CreateUser(CreateUserArgs),
    CloseUser,
}

#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub struct UserInfo {
    pub name: String,
    pub age: u8,
}

impl UserInfo {
    pub const SEED_PREFIX: &'static str = "USERINFO";
}

#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub struct CreateUserArgs {
    pub user_info: UserInfo,
    pub bump: u8,
}
