use crate::instructions::create::create_page_visit;
use crate::instructions::increment::increment_page_visit;
use crate::state::page_visits::PDACommands;
use borsh::BorshDeserialize;
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let command = PDACommands::try_from_slice(data)?;

    match command {
        PDACommands::CreatePageVisits(page_visit_data) => {
            create_page_visit(program_id, accounts, &page_visit_data)
        }
        PDACommands::Increment => increment_page_visit(program_id, accounts),
    }
}
