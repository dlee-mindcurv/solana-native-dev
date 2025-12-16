use crate::state::address_info::AddressInfo;
use crate::state::enhanced_address_info::{EnhancedAddressInfo, EnhancedAddressInfoExtender};
use crate::state::work_info::WorkInfo;
use borsh::{to_vec, BorshDeserialize, BorshSerialize};
use solana_program::account_info::{next_account_info, AccountInfo};
use solana_program::entrypoint::ProgramResult;
use solana_program::msg;
use solana_program::program::invoke;
use solana_program::pubkey::Pubkey;
use solana_program::rent::Rent;
use solana_program::sysvar::Sysvar;

// This method was originally called "without bytes", which technically is true, but the logic in this
// cause is completely rewriting the buffer with the enhanced changes and the original buffer.
// We do the check on the merged object because we need to calculate the space required for the rent
pub fn reallocate_with_original_data(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    enhanced_address_info: EnhancedAddressInfoExtender,
) -> ProgramResult {
    msg!("ReallocInstruction::ReallocateWithoutZeroInit");

    // make the accounts slice iterable
    let accounts_iter = &mut accounts.iter();

    // define payer
    let payer_account = next_account_info(accounts_iter)?;
    let user_account = next_account_info(accounts_iter)?;
    let system_program_account = next_account_info(accounts_iter)?;

    // get the current dat struct from the Account
    let account_info = AddressInfo::try_from_slice(&user_account.data.borrow())?;

    // build the enhanced object from the previously saved AddressIno
    let enhanced_account_info = EnhancedAddressInfo::from_address_info(
        account_info,
        enhanced_address_info.state,
        enhanced_address_info.zip,
    );

    // get the account space of the the total bytes
    let account_space = to_vec(&enhanced_account_info)?.len();

    // get lamports of the the fully merged struct
    let lamports_required = Rent::get()?.minimum_balance(account_space);

    // get amount of what should be paid to support the increase
    // if the user already has enough lamports in the account skip the transfer of funds
    // otherwise calculate how much is needed and then transfer amount from payer's address
    let diff_res = lamports_required.checked_sub(user_account.lamports());
    if let Some(diff) = diff_res {
        if diff > 0 {
            // create the instruction to deposit the increased rent amount to the account
            let ix = solana_system_interface::instruction::transfer(
                &payer_account.key,
                &user_account.key,
                diff,
            );

            // invoke the instruction to transfer the amount
            invoke(
                &ix,
                &[
                    payer_account.clone(),
                    user_account.clone(),
                    system_program_account.clone(),
                ],
            )?;
        }
    } else {
        msg!("USER ALREADY HAS ENOUGH LAMPORTS")
    }

    // resize the data buffer of the account WITHOUT ZERO INITIALIZING
    user_account.resize(account_space)?;

    // write the Enhanced Address changes to the account
    enhanced_account_info.serialize(&mut &mut user_account.data.borrow_mut()[..])?;

    Ok(())
}

//This function was originally called "zero init", however the example never actually 0 initializes
// byte array but rather it just completely overwrites the buffer with new set of bytes "WorkInfo"
pub fn reallocate_new_data_object(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    work_info: WorkInfo,
) -> ProgramResult {
    msg!("ReallocInstruction::ReallocateZeroInit");
    let accounts_iter = &mut accounts.iter();
    let payer = next_account_info(accounts_iter)?;
    let user = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    let new_data_len = to_vec(&work_info)?.len();
    let lamports_required = Rent::get()?.minimum_balance(new_data_len);

    msg!("lamports_required {:?}", lamports_required);
    msg!("user.lamports() {:?}", user.lamports());

    // Checked sub does a check to see if an "overflow" occurs, meaning
    // in this case for a u64 (unsigned) if the difference is negative the falsy
    // condition passes
    let diff_res = lamports_required.checked_sub(user.lamports());

    if let Some(diff) = diff_res {
        if diff > 0 {
            msg!("do a transfer");

            let transfer_ix =
                solana_system_interface::instruction::transfer(&payer.key, &user.key, diff);

            invoke(
                &transfer_ix,
                &[payer.clone(), user.clone(), system_program.clone()],
            )?;
        }
    } else {
        msg!("No transfer required., {:?}")
    }

    user.resize(new_data_len)?;
    work_info.serialize(&mut &mut user.data.borrow_mut()[..])?;

    Ok(())
}
