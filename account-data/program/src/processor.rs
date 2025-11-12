use borsh::BorshDeserialize;
use solana_program::{
    pubkey::Pubkey,
    entrypoint::ProgramResult,
    account_info::AccountInfo,
    program_error::ProgramError
};
use crate::state::AddressInfo;
use crate::instructions;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts:&[AccountInfo],
    program_data: &[u8]
) -> ProgramResult {

    if let Ok(address_info) = AddressInfo::try_from_slice(program_data) {
        return instructions::create_address_info(program_id, accounts, address_info);
    };

    Err(ProgramError::InvalidInstructionData)


}