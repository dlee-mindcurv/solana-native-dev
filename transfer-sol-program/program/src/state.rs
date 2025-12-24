use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub enum TransferCommand {
    Cpi(u64),
    ProgramTransfer(u64),
}
