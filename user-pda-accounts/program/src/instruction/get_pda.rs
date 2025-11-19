use crate::state::Favorites;
use borsh::BorshDeserialize;
use solana_program::account_info::{next_account_info, AccountInfo};
use solana_program::entrypoint::ProgramResult;
use solana_program::msg;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;

pub fn get_pda(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let account_iter = &mut accounts.iter();
    let user = next_account_info(account_iter)?;
    let favorite_account = next_account_info(account_iter)?;

    let (favorite_pda, _) =
        Pubkey::find_program_address(&[b"favorite", user.key.as_ref()], program_id);

    if (&favorite_pda != favorite_account.key) {
        return Err(ProgramError::IncorrectProgramId);
    }

    let favorites = Favorites::try_from_slice(&favorite_account.data.borrow())?;

    msg!(
        "User {}'s favorite number is {}, favorite color is: {}, and their hobbies are {:#?}",
        user.key,
        favorites.number,
        favorites.color,
        favorites.hobbies
    );

    Ok(())
}
