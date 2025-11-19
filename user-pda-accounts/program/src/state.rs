use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Favorites {
    pub number: u64,
    pub color: String,
    pub hobbies: Vec<String>,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Favorites2 {
    pub number: u64,
}
