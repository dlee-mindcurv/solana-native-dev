use borsh::{BorshDeserialize, BorshSerialize};

pub enum User {
    CreatePda(UserInfo),
    SayHello(UserInfo),
}

#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub struct UserInfo {
    pub name: String,
    pub age: u8,
}
