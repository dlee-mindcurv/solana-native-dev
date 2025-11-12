pub mod processor;
pub mod instructions;
pub mod state;

use solana_program::entrypoint;
use processor::process_instruction;


entrypoint!(process_instruction);


