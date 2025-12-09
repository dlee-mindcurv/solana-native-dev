use crate::state::page_visits::PageVisits;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::account_info::{next_account_info, AccountInfo};
use solana_program::entrypoint::ProgramResult;
use solana_program::msg;
use solana_program::pubkey::Pubkey;

pub fn increment_page_visit(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    msg!("increment_page_visit");

    // assign the passed in accounts
    let accounts_iter = &mut accounts.iter();
    let _payer = next_account_info(accounts_iter)?;
    let _user = next_account_info(accounts_iter)?;
    let pda_account = next_account_info(accounts_iter)?;

    // Deserialize the byte-array data from the pda account.  We need to borrow here.
    let mut page_visit = PageVisits::try_from_slice(&pda_account.data.borrow())?;

    // increment the struct
    page_visit.page_visits += 1;

    //
    let mut raw_data = pda_account.data.borrow_mut();

    page_visit.serialize(&mut *raw_data)?;

    Ok(())
}
