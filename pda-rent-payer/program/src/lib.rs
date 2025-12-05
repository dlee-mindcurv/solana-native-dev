use crate::state::RentInstruction::CreatePDA;
use crate::state::{CreatePdaInstruction, RentInstruction, User};
use borsh::{to_vec, BorshDeserialize, BorshSerialize};
use solana_program::account_info::{next_account_info, AccountInfo};
use solana_program::entrypoint_deprecated::ProgramResult;
use solana_program::program::invoke_signed;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use solana_program::rent::Rent;
use solana_program::sysvar::Sysvar;
use solana_program::{entrypoint, msg};

pub mod state;

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    let match_instruction = RentInstruction::try_from_slice(_instruction_data)?;

    match match_instruction {
        RentInstruction::CreatePDA(creation_data) => {
            create_pda(program_id, accounts, &creation_data)
        }
        RentInstruction::DepositLamports(lamport_amount) => deposit_lamports(lamport_amount),
    }

    // Ok(())
}

fn create_pda(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    creation_data: &CreatePdaInstruction,
) -> ProgramResult {
    msg!("CreatePda: {:?}", creation_data);

    let CreatePdaInstruction { payload, bump } = creation_data;

    // iterate through the accounts and assign values
    let accounts_iter = &mut accounts.iter();
    let payer = next_account_info(accounts_iter)?;
    let pda_account = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    // Create the PDA using the bump
    let derived_pda = Pubkey::create_program_address(
        &[b"pda-rent-payer", &payer.key.as_ref(), &[*bump]],
        program_id,
    )?;

    msg!("Derived PDA: {:?}", derived_pda);
    msg!("PDA Account: {:?}", pda_account.key);

    if derived_pda != *pda_account.key {
        return Err(ProgramError::InvalidSeeds);
    };

    let account_span = to_vec(&payload)?.len();
    let lamports = Rent::get()?.minimum_balance(account_span);

    let ix = solana_system_interface::instruction::create_account(
        payer.key,
        pda_account.key,
        lamports,
        account_span as u64,
        program_id,
    );

    invoke_signed(
        &ix,
        &[payer.clone(), pda_account.clone(), system_program.clone()],
        &[&[b"pda-rent-payer", &payer.key.as_ref(), &[*bump]]],
    )?;

    payload.serialize(&mut &mut pda_account.data.borrow_mut()[..])?;

    Ok(())
}

fn deposit_lamports(lamport_amount: usize) -> ProgramResult {
    msg!("deposit_lamports: {:?}", lamport_amount);

    Ok(())
}
