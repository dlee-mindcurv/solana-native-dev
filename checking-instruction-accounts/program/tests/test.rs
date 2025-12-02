use borsh::{to_vec, BorshDeserialize, BorshSerialize};
use litesvm::LiteSVM;
use solana_keypair::{Keypair, Signer};
use solana_native_token::LAMPORTS_PER_SOL;
use solana_pubkey::Pubkey;
use solana_system_interface::instruction::create_account;
use solana_transaction::{AccountMeta, Instruction, Transaction};
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct DummyData {
    name: String,
}

#[test]
pub fn test_entry() {
    // let mut svm = LiteSVM::new();
    //
    // let payer = Keypair::new();
    // let account_to_change = Keypair::new();
    // let account_to_create = Keypair::new();
    //
    // svm.airdrop(&payer.pubkey(), LAMPORTS_PER_SOL * 10).unwrap();
    //
    // let program_id = Pubkey::new_unique();
    // let program_so = include_bytes!("../../tests/fixtures/checking_instruction_accounts.so");
    // svm.add_program(program_id, program_so).unwrap();
    //
    // // Add an account with some lamports
    // let create_account_ix = create_account(
    //     &payer.pubkey(),
    //     &account_to_change.pubkey(),
    //     LAMPORTS_PER_SOL,
    //     0,
    //     &program_id,
    // );
    //
    // let tx = Transaction::new_signed_with_payer(
    //     &[create_account_ix],
    //     Some(&payer.pubkey()),
    //     &[&payer, &account_to_change],
    //     svm.latest_blockhash(),
    // );
    //
    // //verify that the account was created successfully
    // assert!(svm.send_transaction(tx).is_ok(), "Transaction not ok");
    //
    // let serialized_data = DummyData {
    //     name: "David".to_string(),
    // };
    //
    // let instruction_data = to_vec(&serialized_data).unwrap();
    //
    // let ix = Instruction {
    //     program_id,
    //     accounts: vec![
    //         AccountMeta::new(payer.pubkey(), true),
    //         AccountMeta::new(account_to_create.pubkey(), true),
    //         AccountMeta::new(account_to_change.pubkey(), true),
    //         AccountMeta::new(solana_system_interface::programs::ID, false),
    //     ],
    //     data: instruction_data,
    // };
    //
    // let tx = Transaction::new_signed_with_payer(
    //     &[ix],
    //     Some(&payer.pubkey()),
    //     &[payer, account_to_change, account_to_create],
    //     svm.latest_blockhash(),
    // );
    //
    // assert!(
    //     svm.send_transaction(tx).is_ok(),
    //     "transaction with programs failed"
    // )
}
