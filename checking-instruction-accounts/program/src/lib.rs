use processor::process_program;
use solana_program::entrypoint;

mod processor;

entrypoint!(process_program);
