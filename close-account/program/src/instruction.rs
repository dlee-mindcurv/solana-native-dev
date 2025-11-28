use crate::state::{CreateUserArgs, UserInfo};
use borsh::{to_vec, BorshSerialize};
use solana_program::account_info::{next_account_info, AccountInfo};
use solana_program::entrypoint::ProgramResult;
use solana_program::msg;
use solana_program::program::invoke_signed;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use solana_program::rent::Rent;
use solana_program::sysvar::Sysvar;

pub fn create_user(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    create_user_args: CreateUserArgs,
) -> ProgramResult {
    msg!("Create the User: {:#?}", create_user_args);
    let accounts_iter = &mut accounts.iter();

    let payer = next_account_info(accounts_iter)?;
    let pda_account = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    let CreateUserArgs { user_info, bump } = create_user_args;

    let account_span = to_vec(&user_info)?.len();
    let lamports_required = Rent::get()?.minimum_balance(account_span);

    let expected_pda =
        Pubkey::create_program_address(&[b"USERINFO", payer.key.as_ref(), &[bump]], program_id)?;

    if &expected_pda != pda_account.key {
        return Err(ProgramError::InvalidSeeds);
    }

    let create_account_tx = solana_system_interface::instruction::create_account(
        payer.key,
        pda_account.key,
        lamports_required,
        account_span as u64,
        program_id,
    );

    invoke_signed(
        &create_account_tx,
        &[payer.clone(), pda_account.clone(), system_program.clone()],
        &[&[
            UserInfo::SEED_PREFIX.as_bytes(),
            payer.key.as_ref(),
            &[bump],
        ]],
    )?;

    user_info.serialize(&mut &mut pda_account.data.borrow_mut()[..])?;

    Ok(())
}

pub fn close_user(_program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let payer = next_account_info(accounts_iter)?;
    let pda_account = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    let account_span = 0usize;
    let lamports_required = Rent::get()?.minimum_balance(account_span);

    let diff = pda_account.lamports() - lamports_required;

    // send the rent back to the payer.
    **pda_account.lamports.borrow_mut() -= diff;
    **payer.lamports.borrow_mut() += diff;

    // Realloc the account to zero
    pda_account.resize(account_span)?;

    // Assign the account to the System Program
    pda_account.assign(system_program.key);

    Ok(())
}
