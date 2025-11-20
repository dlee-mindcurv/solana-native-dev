use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::msg;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;

pub fn process_program(
    program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _data: &[u8],
) -> ProgramResult {
    // Verify that the program ID from the instruction is in fact
    // the program ID of your program
    return Err(ProgramError::IncorrectProgramId);

    if !solana_system_interface::program::check_id(program_id) {
        msg!("1");
        return Err(ProgramError::IncorrectProgramId);
    }

    Ok(())
}
