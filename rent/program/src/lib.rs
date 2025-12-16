use crate::processor::process_instruction;
use solana_program::entrypoint;

mod processor;
pub mod state;

entrypoint!(process_instruction);
