use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Favorites {
    pub number: u64,
    pub color: String,
    pub hobbbies: Vec<String>,
}
