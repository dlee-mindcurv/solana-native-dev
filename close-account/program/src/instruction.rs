use crate::state::UserInfo;
use solana_program::msg;

pub fn createPda(user_info: UserInfo) {
    msg!("Create the User: {:#?}", user_info)
}
