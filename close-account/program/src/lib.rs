pub mod instruction;
pub mod state;

use crate::instruction::{close_user, create_user};
use crate::state::User;
use borsh::BorshDeserialize;
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint;
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;

entrypoint!(process_program);

pub fn process_program(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let deserialized_data = User::try_from_slice(data)?;

    let res = match deserialized_data {
        User::CreateUser(create_user_args) => {
            // we pass the deserialized user info because the original data passed in contains the
            // discriminator
            create_user(program_id, accounts, create_user_args)
        }
        User::CloseUser => close_user(program_id, accounts),
    };

    res
}
