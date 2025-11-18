use processor::process_instruction;
use solana_program::entrypoint;

mod instruction;
mod processor;
mod state;

entrypoint!(process_instruction);
