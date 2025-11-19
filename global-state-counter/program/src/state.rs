use borsh::{BorshDeserialize, BorshSerialize};

// assign serialization / deserialization traits to struct, debug and clone s well
#[derive(Debug, Copy, Clone, BorshSerialize, BorshDeserialize)]
pub struct Counter {
    pub count: u64,
}

