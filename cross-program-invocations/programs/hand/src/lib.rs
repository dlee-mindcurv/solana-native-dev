use solana_program::account_info::{next_account_info, AccountInfo};
use solana_program::entrypoint;
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;

entrypoint!(process_hand);

pub fn process_hand(program_id: &Pubkey, accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let power = next_account_info(accounts_iter)?;
    let lever_program = next_account_info(accounts_iter)?;

    // let set_power_status_instruction=

    Ok(())
}
