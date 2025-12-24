use borsh::to_vec;
use litesvm::LiteSVM;
use solana_instruction::{AccountMeta, Instruction};
use solana_keypair::{Keypair, Signer};
use solana_program::native_token::LAMPORTS_PER_SOL;
use solana_pubkey::Pubkey;
use solana_transaction::Transaction;
use transfer_sol_program::state::TransferCommand;

#[test]
pub fn transfer_so_test() {
    let mut svm = LiteSVM::new();
    let program_id = Pubkey::new_unique();

    svm.add_program_from_file(program_id, "target/deploy/transfer_sol_program.so")
        .unwrap();

    let system_account_payer = Keypair::new();
    let system_account_receiver = Keypair::new();

    // create a program_account
    let program_account = Keypair::new();

    // airdrop lamports to payer
    svm.airdrop(&system_account_payer.pubkey(), LAMPORTS_PER_SOL * 10)
        .unwrap();

    let create_ix = solana_system_interface::instruction::create_account(
        &system_account_payer.pubkey(),
        &program_account.pubkey(),
        1000000,
        0,
        &program_id,
    );

    let create_tx = Transaction::new_signed_with_payer(
        &[create_ix],
        Some(&system_account_payer.pubkey()),
        &[&system_account_payer, &program_account],
        svm.latest_blockhash(),
    );
    svm.send_transaction(create_tx).unwrap();

    let receiver_account_bal = svm.get_balance(&program_account.pubkey()).unwrap();

    let ix = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(system_account_payer.pubkey(), true),
            AccountMeta::new(system_account_receiver.pubkey(), false),
            AccountMeta::new(solana_system_interface::program::ID, false),
        ],
        data: to_vec(&TransferCommand::Cpi(1000000)).unwrap(),
    };

    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&system_account_payer.pubkey()),
        &[&system_account_payer],
        svm.latest_blockhash(),
    );

    let tx_res = svm.send_transaction(tx);

    assert!(tx_res.is_ok());

    for logs in &tx_res.unwrap().logs {
        println!("{:?}", logs);
    }

    let system_account_receiver_account_bal =
        svm.get_balance(&system_account_receiver.pubkey()).unwrap();

    assert_eq!(system_account_receiver_account_bal, 1000000);

    let ix_return = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(program_account.pubkey(), true),
            AccountMeta::new(system_account_receiver.pubkey(), false),
            AccountMeta::new(solana_system_interface::program::ID, false),
        ],
        data: to_vec(&TransferCommand::ProgramTransfer(90000)).unwrap(),
    };

    let tx_return = Transaction::new_signed_with_payer(
        &[ix_return],
        Some(&system_account_payer.pubkey()),
        &[&system_account_payer, &program_account],
        svm.latest_blockhash(),
    );

    let tx_return_result = svm.send_transaction(tx_return);

    assert!(
        tx_return_result.is_ok(),
        "Return transfer transaction failed: {:?}",
        tx_return_result.err()
    );

    for logs in &tx_return_result.unwrap().logs {
        println!("{:?}", logs);
    }

    let program_account_bal = svm.get_balance(&program_account.pubkey()).unwrap();
    let system_account_receiver_bal = svm.get_balance(&system_account_receiver.pubkey()).unwrap();

    println!("Program account balance {}", program_account_bal);
    println!("Receiver account balance {}", system_account_receiver_bal);

    // program account doesn't apy the fee because he transfer is to a system account
    assert_eq!(program_account_bal, 910000);
    assert_eq!(system_account_receiver_bal, 1090000);
}
