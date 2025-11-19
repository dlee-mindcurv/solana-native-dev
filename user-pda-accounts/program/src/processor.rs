use crate::instruction::create_pda::create_pda;
use crate::instruction::get_pda::get_pda;
use crate::state::Favorites;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub enum FavoritesInstruction {
    CreatePda(Favorites),
    GetPda,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, PartialOrd, PartialEq)]
struct MyStruct {
    instruction: u8,
    number: u64,
    color: String,
    hobbies: Vec<String>,
}

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = FavoritesInstruction::try_from_slice(instruction_data)?;

    match instruction {
        FavoritesInstruction::CreatePda(data) => {
            create_pda(program_id, accounts, data)
        }
        FavoritesInstruction::GetPda => get_pda(program_id, accounts)
    }?;

    Ok(())
}
