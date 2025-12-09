use crate::state::ProgramInstruction;
use borsh::BorshDeserialize;
use solana_program::account_info::next_account_info;
use solana_program::entrypoint::ProgramResult;
use solana_program::{entrypoint, msg};

pub mod state;

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &solana_program::pubkey::Pubkey,
    accounts: &[solana_program::account_info::AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // organize account

    let accounts_iter = &mut accounts.iter();
    let payer = next_account_info(accounts_iter)?;
    let pda_account = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    let instruction = ProgramInstruction::try_from_slice(instruction_data)?;

    match instruction {
        ProgramInstruction::CreateUser(user_data) => msg!("Create User{:?}", user_data),
        ProgramInstruction::CreateStudent(student_data) => msg!("Create Student{:?}", student_data),
    }

    return Ok(());
}
