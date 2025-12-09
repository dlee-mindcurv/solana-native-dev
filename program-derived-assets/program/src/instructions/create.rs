use crate::state::page_visits::PageVisits;
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint_deprecated::ProgramResult;
use solana_program::msg;
use solana_program::pubkey::Pubkey;

pub fn create_page_visit(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &PageVisits,
) -> ProgramResult {
    msg!("create_page_visit {:?}", data);
    Ok(())
}
