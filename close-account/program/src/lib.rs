pub mod instruction;
pub mod state;

use solana_program::account_info::{next_account_info, AccountInfo};
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;
use solana_program::{entrypoint, msg};

entrypoint!(process_program);

pub fn process_program(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    _data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let payer = next_account_info(accounts_iter)?;
    let pda_account = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    msg!("{:?}", payer.key);
    // msg!("{:?}", pda_account.key);
    // msg!("{:?}", system_program.key);

    Ok(())
}
