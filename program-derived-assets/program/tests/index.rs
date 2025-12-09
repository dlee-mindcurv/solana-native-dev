use borsh::to_vec;
use litesvm::LiteSVM;
use program_derived_assets::state::page_visits::{PDACommands, PageVisits};
use solana_instruction::{AccountMeta, Instruction};
use solana_keypair::{Keypair, Signer};
use solana_program::native_token::LAMPORTS_PER_SOL;
use solana_pubkey::Pubkey;
use solana_transaction::Transaction;

#[test]
pub fn test_process_instruction() {
    // define mutable svm
    let mut svm = LiteSVM::new();

    // define id for program
    let program_id = Pubkey::new_unique();

    // add program to the svm
    svm.add_program_from_file(program_id, "target/deploy/program_derived_assets.so")
        .unwrap();

    // create a test payer Keypair
    let payer = Keypair::new();

    // airdrop some tokens to this new Account
    svm.airdrop(&payer.pubkey(), LAMPORTS_PER_SOL).unwrap();

    // find the pda_account for the program
    // this account will allow anyone to update the pda as there
    let (pda_account, bump) = Pubkey::find_program_address(&[b"page_visits"], &program_id);

    let page_visit = PageVisits {
        page_visits: 10,
        bump,
    };

    // create command for the create instruction
    let create_page_visit_data = to_vec(&PDACommands::CreatePageVisits(page_visit)).unwrap();

    // definie for the create tx
    let ix_create = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new(pda_account, false),
            AccountMeta::new_readonly(solana_system_interface::program::ID, false),
        ],
        data: create_page_visit_data,
    };

    let increment_command_data = to_vec(&PDACommands::Increment).unwrap();

    // defined the instruction for the increment
    let ix_increment = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new(pda_account, false),
            AccountMeta::new(solana_system_interface::program::ID, false),
        ],
        data: increment_command_data,
    };

    // define the tx for the instructions
    let page_visit_instructions = Transaction::new_signed_with_payer(
        &[ix_create, ix_increment],
        Some(&payer.pubkey()),
        &[payer],
        svm.latest_blockhash(),
    );

    // execute transactions
    let tx_res = svm.send_transaction(page_visit_instructions).unwrap();

    for log in tx_res.logs {
        println!("{:?}", log);
    }
}
