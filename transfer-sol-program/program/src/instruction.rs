use solana_program::account_info::{next_account_info, AccountInfo};
use solana_program::entrypoint::ProgramResult;
use solana_program::program::invoke;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;

pub fn cpi_transfer(program_id: &Pubkey, accounts: &[AccountInfo], lamports: u64) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let payer = next_account_info(accounts_iter)?;
    let receiver = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    let ix = solana_system_interface::instruction::transfer(&payer.key, &receiver.key, lamports);

    invoke(
        &ix,
        &[payer.clone(), receiver.clone(), system_program.clone()],
    )?;
    Ok(())
}

pub fn program_transfer(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    lamports: u64,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let payer = next_account_info(accounts_iter)?;
    let receiver = next_account_info(accounts_iter)?;

    if (payer.owner != program_id) {
        return Err(ProgramError::IllegalOwner);
    }

    **payer.try_borrow_mut_lamports()? -= lamports;
    **receiver.try_borrow_mut_lamports()? += lamports;

    Ok(())
}
