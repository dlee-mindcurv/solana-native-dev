use borsh::{to_vec, BorshDeserialize};
use lever::SetPowerStatus;
use solana_program::account_info::{next_account_info, AccountInfo};
use solana_program::entrypoint;
use solana_program::entrypoint::ProgramResult;
use solana_program::instruction::{AccountMeta, Instruction};
use solana_program::program::invoke;
use solana_program::pubkey::Pubkey;

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    // assign the account that with provide "power (on/off) to the lever"
    let power_account = next_account_info(accounts_iter)?;

    // assign the address for the lever program
    let lever_program = next_account_info(accounts_iter)?;

    // deserialize the instruction data into a BASE struct
    let set_power_status_instruction = SetPowerStatus::try_from_slice(data)?;

    // create an instruction to send to the Lever program
    let ix = Instruction {
        program_id: *lever_program.key,
        accounts: vec![AccountMeta::new(*power_account.key, false)],
        data: to_vec(&set_power_status_instruction).unwrap(),
    };

    invoke(&ix, &[power_account.clone()]);

    Ok(())
}
