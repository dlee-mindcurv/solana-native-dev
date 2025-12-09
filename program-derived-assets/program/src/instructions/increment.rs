use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::msg;
use solana_program::pubkey::Pubkey;

pub fn increment_page_visit(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    msg!("increment_page_visit");
    Ok(())
}
