use borsh::{to_vec, BorshDeserialize, BorshSerialize};
use lever::{PowerStatus, SetPowerStatus};
use solana_program::account_info::{next_account_info, AccountInfo};
use solana_program::entrypoint::ProgramResult;
use solana_program::instruction::{AccountMeta, Instruction};
use solana_program::program::invoke;
use solana_program::pubkey::Pubkey;
use solana_program::{entrypoint, msg};

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    // skip first account assignment (payer)
    // let _ = next_account_info(accounts_iter)?;

    // assign the account that with provide "power (on/off) to the lever"
    let power_account = next_account_info(accounts_iter)?;

    // assign the address for the lever program
    let lever_program = next_account_info(accounts_iter)?;

    // Deserialize the data
    let set_power_status_instruction = SetPowerStatus::try_from_slice(data)?;

    msg!(
        "HAND::set_power_status_instruction {:?}",
        set_power_status_instruction
    );

    // create an instruction that calls the lever program with the operation
    let ix = Instruction::new_with_bytes(
        *lever_program.key,
        to_vec(&set_power_status_instruction).unwrap().as_ref(),
        vec![AccountMeta::new(*power_account.key, false)],
    );

    invoke(&ix, &[power_account.clone()])
}
