use borsh::{BorshDeserialize, BorshSerialize};
#[derive(Debug, BorshDeserialize, BorshSerialize, PartialEq, Eq)]
pub enum Pick {
    England,
    Italy,
    Netherlands,
}

#[derive(Debug, BorshDeserialize, BorshSerialize, PartialEq, Eq)]
pub struct UserPick {
    pub pick1: Pick,
    pub pick2: Pick,
}

#[derive(Debug, BorshDeserialize, BorshSerialize, PartialEq, Eq)]
pub struct PDAAllocator {
    pub bump: u8,
}

impl UserPick {
    pub fn new(pick1: Pick, pick2: Pick) -> UserPick {
        UserPick { pick1, pick2 }
    }
}

#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub enum InstructionCommand {
    Add(UserPick, PDAAllocator),
    Remove,
}
