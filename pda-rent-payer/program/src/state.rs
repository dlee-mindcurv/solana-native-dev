use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub struct CreatePdaInstruction {
    pub bump: u8,
    pub payload: User,
}

#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub enum RentInstruction {
    CreatePDA(CreatePdaInstruction),
    DepositLamports(usize),
}

#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub struct User {
    pub name: String,
    pub age: u8,
}
