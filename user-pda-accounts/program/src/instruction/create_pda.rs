use crate::state::Favorites;
use borsh::BorshSerialize;
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
    let favorite_account = next_account_info(account_iter)?;
    let system_program = next_account_info(account_iter)?;

    // generate the pda based off a static string byte array and the users PubKey (byte Array)
    let (favorite_pda, favorite_bump) =
        Pubkey::find_program_address(&[b"favorite", user.key.as_ref()], program_id);

    // check if the favorite account is the same as the derived favorite pda
    if favorite_account.key != &favorite_pda {
        return Err(ProgramError::IncorrectProgramId);
    }

    // check if the pda is already initialized
    if favorite_account.data.borrow().len() == 0 {
        // initialize the favorite account
        let space = borsh::to_vec(&data)?.len();
        let lamports = Rent::get()?.minimum_balance(space);

        // create the account for the transaction
        let ix = solana_system_interface::instruction::create_account(
            user.key,
            favorite_account.key,
            lamports,
            space as u64,
            program_id,
        );
        invoke_signed(
            &ix,
            &[
                user.clone(),
                favorite_account.clone(),
                system_program.clone(),
            ],
            &[&[b"favorite", user.key.as_ref(), &[favorite_bump]]],
        )?;
        data.serialize(&mut &mut favorite_account.data.borrow_mut()[..])?;
    } else {
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    Ok(())
}
