use crate::state::page_visits::PageVisits;
use borsh::to_vec;
use solana_program::account_info::{next_account_info, AccountInfo};
use solana_program::entrypoint_deprecated::ProgramResult;
use solana_program::msg;
use solana_program::program::invoke_signed;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use solana_program::rent::Rent;
use solana_program::sysvar::Sysvar;

pub fn create_page_visit(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &PageVisits,
) -> ProgramResult {
    msg!("create_page_visit {:?}", data);

    // assign the passed in accounts
    let accounts_iter = &mut accounts.iter();
    let payer = next_account_info(accounts_iter)?;
    let user = next_account_info(accounts_iter)?;
    let pda_account = next_account_info(accounts_iter)?;
    let solana_program_account = next_account_info(accounts_iter)?;

    // verify that the pda sent through is matching
    let derived_pda = Pubkey::create_program_address(
        &[b"page_visits", &user.key.as_ref(), &[data.bump]],
        program_id,
    )?;

    // compare the two addresses to ensure that they match
    if derived_pda != *pda_account.key {
        return Err(ProgramError::InvalidSeeds);
    }

    // calculate the space needed in bytes
    let account_span = to_vec(data)?.len();

    // calculate the cost "rent" instoring these bytes
    let lamports = Rent::get()?.minimum_balance(account_span);

    // create the PDA account instruction
    let create_pda_ix = solana_system_interface::instruction::create_account(
        &payer.key,
        &derived_pda,
        lamports,
        account_span as u64,
        program_id,
    );

    //invoke the account
    invoke_signed(
        &create_pda_ix,
        &[
            payer.clone(),
            user.clone(),
            pda_account.clone(),
            solana_program_account.clone(),
        ],
        &[&[b"page_visits", &payer.key.as_ref(), &[data.bump]]],
    )

    // * as the initial storage length is zero, we will not save any data here
}
