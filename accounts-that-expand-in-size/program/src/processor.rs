use crate::instructions::create::create_address_info;
use crate::instructions::reallocate::{reallocate_new_data_object, reallocate_with_original_data};
use crate::state::address_info::AddressInfo;
use crate::state::enhanced_address_info::EnhancedAddressInfoExtender;
use crate::state::work_info::WorkInfo;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;

#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub enum ReallocInstruction {
    Create(AddressInfo),
    ReallocateWithoutZeroInit(EnhancedAddressInfoExtender),
    ReallocateZeroInit(WorkInfo),
}

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let instruction = ReallocInstruction::try_from_slice(data)?;

    match instruction {
        ReallocInstruction::Create(address) => {
            return create_address_info(program_id, accounts, address)
        }
        ReallocInstruction::ReallocateWithoutZeroInit(enhancedAddressInfo) => {
            return reallocate_with_original_data(program_id, accounts, enhancedAddressInfo)
        }
        ReallocInstruction::ReallocateZeroInit(work_info) => {
            return reallocate_new_data_object(program_id, accounts, work_info)
        }
    }
}
