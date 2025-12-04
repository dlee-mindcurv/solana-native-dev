use borsh::{to_vec, BorshDeserialize, BorshSerialize};

use solana_program::account_info::{next_account_info, AccountInfo};

#[cfg(not(feature = "no-entrypoint"))]
use solana_program::entrypoint;
use solana_program::entrypoint::ProgramResult;
use solana_program::instruction::{AccountMeta, Instruction};
use solana_program::msg;
use solana_program::program::invoke;
use solana_program::pubkey::Pubkey;
use solana_program::rent::Rent;
use solana_program::sysvar::Sysvar;
use solana_system_interface::instruction::create_account;

#[cfg(not(feature = "no-entrypoint"))]
entrypoint!(process_instruction);

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct SetPowerStatus {
    pub name: String,
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct PowerStatus {
    pub is_on: bool,
}

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    if let Ok(power_status) = PowerStatus::try_from_slice(data) {
        msg!("LEVER INITIALIZE::Power status: {:?}", power_status);
        return initialize(program_id, accounts, power_status);
    }

    if let Ok(set_power_status) = SetPowerStatus::try_from_slice(data) {
        msg!("LEVER::Set Power status: {:?}", set_power_status);
        // return initialize(program_id, accounts, set_power_status);
    }

    // Err(ProgramError::InvalidInstructionData)

    // msg!("LEVER PROGRAM: {:?}", status);
    //
    // match status {
    //     Ok(lever_status) => {
    //         msg!("lever_status {:?}", lever_status)
    //     }
    //     Err(error) => {
    //         msg!("Error {:?}", error)
    //     }
    // }

    Ok(())
}

pub fn initialize(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    power_status: PowerStatus,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let payer = next_account_info(accounts_iter)?;
    let power = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    // get the span of the data
    let account_span = to_vec(&power_status)?.len();
    let lamports_required = Rent::get()?.minimum_balance(account_span);

    let ix = create_account(
        payer.key,
        power.key,
        lamports_required,
        account_span as u64,
        program_id,
    );

    invoke(&ix, &[payer.clone(), power.clone(), system_program.clone()]);

    power_status.serialize(&mut &mut power.data.borrow_mut()[..])?;

    Ok(())
}
