use litesvm::LiteSVM;
use std::str::FromStr;
use solana_program::instruction::Instruction;
use solana_program::message::Message;
use solana_program::pubkey::Pubkey;
use solana_sdk::native_token::LAMPORTS_PER_SOL;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::transaction::Transaction;

#[test]
fn test_it_works() {
    let mut svm = LiteSVM::new();

    let program_keypair = Keypair::new();
    let program_id = program_keypair.pubkey();

    let payer = Keypair::new();
    svm.airdrop(&payer.pubkey(), LAMPORTS_PER_SOL).expect("pass");

    svm.add_program_from_file(program_id, "target/deploy/hello_solana.so")
        .expect("TODO: panic message");

    let instruction = Instruction::new_with_bincode(program_id, &[0], vec![]);

    let message = Message::new(&[instruction], Some(&payer.pubkey()));

    let transaction = Transaction::new(&[&payer], message, svm.latest_blockhash());

    let result = svm.send_transaction(transaction);


    assert!(result.is_ok(),"Transaction should be successful");

}