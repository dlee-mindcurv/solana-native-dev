use crate::instruction::{cpi_transfer, program_transfer};
use crate::state::TransferCommand;
use borsh::BorshDeserialize;
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;

pub fn process_transaction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let data = TransferCommand::try_from_slice(instruction_data)?;

    match data {
        TransferCommand::ProgramTransfer(lamports) => {
            return program_transfer(program_id, accounts, lamports)
        }
        TransferCommand::Cpi(lamports) => return cpi_transfer(program_id, accounts, lamports),
    }
}
