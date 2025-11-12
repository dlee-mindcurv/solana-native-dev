mod state;
use crate::state::Counter;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::account_info::{next_account_info, AccountInfo};
use solana_program::entrypoint::ProgramResult;
use solana_program::program_error::ProgramError;
use solana_program::{entrypoint, msg, pubkey::Pubkey};

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    program_data: &[u8],
) -> ProgramResult {
    let (instruction_discriminant, instruction_data_inner) = program_data.split_at(1);
    match instruction_discriminant[0] {
        0 => {
            msg!("Instruction Increment");
            process_increment_counter(accounts, instruction_data_inner)?;
        }
        _ => {
            msg!("Error: unknown instruction")
        }
    }

    Ok(())
}

pub fn process_increment_counter(
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> Result<(), ProgramError> {
    // 1. iterate through accounts
    let account_info_iter = &mut accounts.iter();

    // assign the counter account as the first index.  Insure account is writeable to incrememnt
    let counter_account = next_account_info(account_info_iter)?;
    assert!(
        counter_account.is_writable,
        "Counter account must be writable"
    );

    // assign the counte by trying to get the prgram data
    let mut counter = Counter::try_from_slice(&counter_account.try_borrow_mut_data()?)?;

    // if the data is successfull there (which it should be from initialization) then increment the counter
    counter.count += 1;
    counter.serialize(&mut *counter_account.data.borrow_mut())?;

    msg!("Counter state incremented to {:?}", counter.count);

    Ok(())
}
