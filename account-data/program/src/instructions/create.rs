use borsh::BorshSerialize;
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;
use solana_program::account_info::next_account_info;
use solana_program::program::invoke;
use solana_program::rent::Rent;
use solana_program::sysvar::Sysvar;
use crate::state::AddressInfo;

pub fn create_address_info(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    address_info: AddressInfo
) -> ProgramResult {

    // get the account iter so we can determing the accounts in order
    let account_iter = &mut accounts.iter();

    let address_info_account = next_account_info(account_iter)?;
    let payer = next_account_info(account_iter)?;
    let system_program = next_account_info(account_iter)?;

    // use borsh to serialize the payload into a vector byte array
    let account_span = borsh::to_vec(&address_info)?.len();

    // get the required lamports for rental cost
    let lamports_required = Rent::get()?.minimum_balance(account_span);

    let ix = solana_system_interface::instruction::create_account(
        payer.key,
        address_info_account.key,
        lamports_required,
        account_span as u64,
        program_id
    );

    invoke(
        &ix,
        &[
            payer.clone(),
            address_info_account.clone(),
            system_program.clone()
        ]
    )?;


    address_info.serialize(&mut &mut address_info_account.data.borrow_mut()[..])?;


    Ok(())

}

