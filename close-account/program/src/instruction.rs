use crate::state::UserInfo;
use solana_program::msg;

pub fn create_pda(user_info: UserInfo) {
    msg!("Create the User: {:#?}", user_info)
}
