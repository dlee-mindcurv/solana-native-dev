use crate::processor::process_transaction;
use solana_program::entrypoint;

mod instruction;
pub mod processor;
pub mod state;

entrypoint!(process_transaction);
