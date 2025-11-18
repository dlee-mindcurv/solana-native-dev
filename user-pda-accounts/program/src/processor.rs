use crate::instruction::create_pda::create_pda;
use crate::instruction::get_pda::get_pda;
use crate::state::Favorites;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;

#[derive(BorshDeserialize, BorshSerialize)]
pub enum FavoritesInstruction {
    CreatePda(Favorites),
    GetPda,
}

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // see of data is being passed into the program...
    let instruction = FavoritesInstruction::try_from_slice(instruction_data)?;

    // If there is data execute first method of the match
    match instruction {
        FavoritesInstruction::CreatePda(data) => create_pda(program_id, accounts, data),
        FavoritesInstruction::GetPda => get_pda(program_id, accounts),
    }?;

    Ok(())
}
