use borsh::{BorshDeserialize, BorshSerialize};

use solana_program::account_info::AccountInfo;

#[cfg(not(feature = "no-entrypoint"))]
use solana_program::entrypoint;
use solana_program::entrypoint::ProgramResult;
use solana_program::msg;
use solana_program::pubkey::Pubkey;

#[cfg(not(feature = "no-entrypoint"))]
entrypoint!(process_instruction);

#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub struct PowerStatus {
    is_on: bool,
}

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let status = PowerStatus::try_from_slice(data);

    if let Ok(set_power_status) = status {
        msg!("Set Power status: {:?}", set_power_status.is_on);
    }

    Ok(())
}
