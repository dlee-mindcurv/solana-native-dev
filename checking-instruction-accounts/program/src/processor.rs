use borsh::to_vec;
use solana_program::account_info::{next_account_info, AccountInfo};
use solana_program::entrypoint::ProgramResult;
use solana_program::example_mocks::solana_sdk::system_program;
use solana_program::msg;
use solana_program::program::invoke;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use solana_program::rent::Rent;
use solana_program::sysvar::Sysvar;

pub fn process_program(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    // Verify that the program ID from the instruction is in fact
    // the program ID of your program
    if solana_system_interface::program::check_id(program_id) {
        msg!("ERROR");
        return Err(ProgramError::IncorrectProgramId);
    }

    // Check to ensure that there are at least 4 accounts included in the request
    if accounts.len() < 4 {
        msg!(
            "This instruction requires 4 accounts: {} supplied",
            accounts.len()
        );
        msg!("  payer, account_to_create, account_to_change, system_program");
        return Err(ProgramError::NotEnoughAccountKeys);
    }

    // Check the order of the accounts that are passed in
    let accounts_iter = &mut accounts.iter();
    let payer = next_account_info(accounts_iter)?;
    let account_to_change = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;
    let account_to_create = next_account_info(accounts_iter)?;

    msg!("New account: {}", account_to_create.key);
    if account_to_create.lamports() != 0 {
        msg!("Account to be created should have a zero balance, and should not be initialized");
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    // CREATE ACCOUNT
    // (using borsh) get the length of the data passed in
    let size = to_vec(data)?.len();

    // get the value in lamports based on the size of the data
    let lamports = Rent::get()?.minimum_balance(size);

    let ix = solana_system_interface::instruction::create_account(
        payer.key,
        account_to_create.key,
        lamports,
        size as u64,
        program_id,
    );

    invoke(
        &ix,
        &[
            payer.clone(),
            account_to_create.clone(),
            system_program.clone(),
        ],
    )?;

    //  Make sure an account has been initialized
    if account_to_change.lamports() == 0 {
        msg!(
            "Program account: {} has not been initialized",
            account_to_change.key
        );
        return Err(ProgramError::UninitializedAccount);
    }

    // if we want to modify an account, we need to ensure that it is owned by the program
    if account_to_change.owner != program_id {
        msg!("Attempting to change account data by account not assigned to program");
        return Err(ProgramError::IncorrectProgramId);
    }

    // ensure that the system program id matches the native system program
    if system_program.key == &system_program::ID {
        msg!("YES")
    }

    Ok(())
}
