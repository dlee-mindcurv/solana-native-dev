use crate::state::address_info::AddressInfo;
use borsh::{to_vec, BorshSerialize};
use solana_program::account_info::{next_account_info, AccountInfo};
use solana_program::entrypoint::ProgramResult;
use solana_program::msg;
use solana_program::program::invoke;
use solana_program::pubkey::Pubkey;
use solana_program::rent::Rent;
use solana_program::sysvar::Sysvar;

pub fn create_address_info(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    address_info: AddressInfo,
) -> ProgramResult {
    msg!("ReallocInstruction::Create");

    // make the accounts slice iterable
    let accounts_iter = &mut accounts.iter();

    // define payer
    let payer_account = next_account_info(accounts_iter)?;
    let user_account = next_account_info(accounts_iter)?;
    let system_program_account = next_account_info(accounts_iter)?;

    // define the length in bytes for the account data
    let account_span = to_vec(&address_info)?.len();

    // define the lamports to be assigned
    let lamports = (Rent::get()?).minimum_balance(account_span);

    let create_account_ix = solana_system_interface::instruction::create_account(
        payer_account.key,
        user_account.key,
        lamports,
        account_span as u64,
        program_id,
    );

    invoke(
        &create_account_ix,
        &[
            payer_account.clone(),
            user_account.clone(),
            system_program_account.clone(),
        ],
    );

    let original = &mut &mut user_account.data.borrow_mut()[..];
    address_info.serialize(original)?;

    Ok(())
}
