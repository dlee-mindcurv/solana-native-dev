
use solana_program::account_info::{next_account_info, AccountInfo};
use solana_program::{entrypoint, msg};
use solana_program::entrypoint::ProgramResult;
use solana_program::program::invoke;
use solana_program::pubkey::Pubkey;
use solana_program::rent::Rent;
use solana_program::sysvar::Sysvar;

entrypoint!(process_entry);

pub fn process_entry(_program_id: &Pubkey, accounts: &[AccountInfo], data:&[u8]) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let payer = next_account_info(accounts_iter)?;
    let new_account = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    msg!("Program invoked. Creating a system account...");
    msg!("  New public key will be: {}", &new_account.key.to_string());

    let account_span = data.len();
    let lamports_required = Rent::get()?.minimum_balance(account_span);

    let ix = solana_system_interface::instruction::create_account(
        payer.key,
        new_account.key,
        lamports_required,
        account_span as u64,
        &solana_system_interface::program::ID
    );

    invoke(&ix, &[
        payer.clone(),
        new_account.clone(),
        system_program.clone()
    ])?;

    msg!("Account created successfully");
    Ok(())
}