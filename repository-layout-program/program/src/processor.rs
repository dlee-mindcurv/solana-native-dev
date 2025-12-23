use crate::instructions::eat_food::{eat_food, EatFoodInstructionData};
use crate::instructions::get_on_ride::{get_on_ride, GetOnRideInstructionData};
use crate::instructions::play_game::{play_game, PlayGameInstructionData};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::msg;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;

pub enum InstructionCommand {
    EatFood(EatFoodInstructionData),
    GetOnRide(GetOnRideInstructionData),
    PlayGame(PlayGameInstructionData),
}

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct CarnivalInstructionData {
    pub name: String,
    pub height: u32,
    pub ticket_count: u32,
    pub attraction: String,
    pub attraction_name: String,
}

pub fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    instruction_daa: &[u8],
) -> ProgramResult {
    let ix_data_object = CarnivalInstructionData::try_from_slice(instruction_daa)?;

    msg!("Welcome to the carnival, {}!", ix_data_object.name);

    match ix_data_object.attraction.as_str() {
        "ride" => get_on_ride(GetOnRideInstructionData {
            ride: ix_data_object.attraction_name,
            rider_name: ix_data_object.name,
            rider_height: ix_data_object.height,
            rider_ticket_count: ix_data_object.ticket_count,
        }),
        "game" => play_game(PlayGameInstructionData {
            game: ix_data_object.attraction_name,
            gamer_name: ix_data_object.name,
            gamer_ticket_count: ix_data_object.ticket_count,
        }),
        "food" => eat_food(EatFoodInstructionData {
            eater_name: ix_data_object.name,
            food_stand: ix_data_object.attraction_name,
            eater_ticket_count: ix_data_object.ticket_count,
        }),
        _ => Err(ProgramError::InvalidInstructionData),
    }
}
