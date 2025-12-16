use crate::state::{InstructionCommand, UserPick};
use borsh::{to_vec, BorshDeserialize, BorshSerialize};
use solana_program::account_info::{next_account_info, AccountInfo};
use solana_program::entrypoint::ProgramResult;
use solana_program::msg;
use solana_program::program::invoke_signed;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use solana_program::rent::Rent;
use solana_program::sysvar::Sysvar;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let instruction = InstructionCommand::try_from_slice(data)?;

    match instruction {
        InstructionCommand::Add(user_pick, pda_allocator) => {
            return add(program_id, accounts, user_pick, pda_allocator.bump)
        }
        InstructionCommand::Remove => return remove(),
    }
}

pub fn add(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    user_pick: UserPick,
    bump: u8,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let payer = next_account_info(accounts_iter)?;
    let pda = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    // create the derived PDA based on the bump passed in
    let derived_pda =
        Pubkey::create_program_address(&[b"rent", &payer.key.as_ref(), &[bump]], program_id)
            .unwrap();

    // compare the derived account with the account passed in... if the are not equal exit the transaction
    if !derived_pda.eq(&pda.key) {
        return Err(ProgramError::InvalidSeeds);
    }

    // if the two pdas are equal create the account:x

    //1. calcuate the "account span" the length in byxtes of the data
    let account_span = to_vec(&user_pick)?.len();

    //2. calculate the rent on lamports to hold this particular submission
    let lamports = Rent::get()?.minimum_balance(account_span);

    msg!("PDA Address {:?}", pda);
    msg!("account_span (size in bytes):  {:?}", account_span);
    msg!("lamports (cost to hold pda):  {:?}", lamports);

    // create thee instruction to create the PDA for the user (payer)

    let ix = solana_system_interface::instruction::create_account(
        &payer.key,
        &derived_pda,
        lamports,
        account_span as u64,
        program_id,
    );

    // invoke the instruction to create the account
    invoke_signed(
        &ix,
        &[payer.clone(), pda.clone(), system_program.clone()],
        &[&[b"rent", &payer.key.as_ref(), &[bump]]],
    );

    // submit the data changes to the on chain data buffer
    // Serialze the data and then commit it to the borrowed_mut of the account data
    user_pick.serialize(&mut &mut pda.data.borrow_mut()[..])?;

    Ok(())
}

pub fn remove() -> ProgramResult {
    Ok(())
}
