use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::msg;
use solana_program::pubkey::Pubkey;

pub fn process_instruction (
    _program_id:&Pubkey,
    _accounts:&[AccountInfo],
    _instruction_data:&[u8]
) -> ProgramResult {
    msg!("Account program");
    Ok(())
}