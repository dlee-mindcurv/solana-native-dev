use crate::state::Favorites;
use solana_program::account_info::{next_account_info, AccountInfo};
use solana_program::entrypoint_deprecated::ProgramResult;
use solana_program::program::invoke_signed;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use solana_program::rent::Rent;
use solana_program::sysvar::Sysvar;

pub fn create_pda(program_id: &Pubkey, accounts: &[AccountInfo], data: Favorites) -> ProgramResult {
    let account_iter = &mut accounts.iter();
    let user = next_account_info(account_iter)?; // user who is signing tx
    let favourite_account = next_account_info(account_iter)?;
    let system_program = next_account_info(account_iter)?;

    // generate the pda based off a static string byte array and the users PubKey (byte Array)
    let (favourite_pda, favourite_bump) =
        Pubkey::find_program_address(&[b"favorite", user.key.as_ref()], program_id);

    // check if the favourite account is the same as the derived favourite pda
    if favourite_account.key != &favourite_pda {
        return Err(ProgramError::IncorrectProgramId);
    }

    // check if the pda is already initialized
    if favourite_account.data.borrow().len() == 0 {
        // initialize the favourite account
        let space = borsh::to_vec(&data)?.len();
        let lamports = Rent::get()?.minimum_balance(space);

        // create the account for the transaction
        let ix = solana_system_interface::instruction::create_account(
            user.key,
            favourite_account.key,
            lamports,
            space as u64,
            program_id,
        );

        invoke_signed(
            &ix,
            &[
                user.clone(),
                favourite_account.clone(),
                system_program.clone(),
            ],
            &[&[b"favourite", user.key.as_ref(), &[favourite_bump]]],
        )?;
    } else {
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    Ok(())
}
