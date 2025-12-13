use crate::state::address_info::AddressInfo;
use crate::state::enhanced_address_info::{EnhancedAddressInfo, EnhancedAddressInfoExtender};
use borsh::{to_vec, BorshDeserialize, BorshSerialize};
use solana_program::account_info::{next_account_info, AccountInfo};
use solana_program::entrypoint::ProgramResult;
use solana_program::msg;
use solana_program::program::invoke;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use solana_program::rent::Rent;
use solana_program::sysvar::Sysvar;

pub fn reallocate_without_zero_init(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    enhancedAddressInfo: EnhancedAddressInfoExtender,
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
        enhancedAddressInfo.state,
        enhancedAddressInfo.zip,
    );

    // get the account space of the the total bytes
    let account_space = to_vec(&enhanced_account_info)?.len();

    // get lamports of the the fully merged struct
    let lamports_required = Rent::get()?.minimum_balance(account_space);

    // get amount of what should be paid to support the increase
    // if the user alread has enough lamports in the account skip the transfer of funds
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

pub fn reallocate_zero_init() -> ProgramResult {
    msg!("ReallocInstruction::ReallocateZeroInit");
    Ok(())
}
