use litesvm::LiteSVM;
use solana_program::instruction::AccountMeta;
use solana_sdk::message::Instruction;
use solana_sdk::native_token::LAMPORTS_PER_SOL;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{Keypair, Signer};
use solana_sdk::transaction::Transaction;

#[test]
pub fn test_create_account() {
    let payer = Keypair::new();
    let new_account = Keypair::new();
    let program_id =  Pubkey::new_unique();

    let mut svm = LiteSVM::new();
    svm.add_program_from_file(program_id,"./target/deploy/create_account.so").unwrap();
    svm.airdrop(&payer.pubkey(), LAMPORTS_PER_SOL * 10).unwrap();

    let player_balance =  svm.get_balance(&payer.pubkey());
    assert!(player_balance.is_some());

    let new_account_balance = svm.get_balance(&new_account.pubkey());
    assert!(new_account_balance.is_none());

    let ix  = Instruction::new_with_bytes(program_id, &[0], vec![
        AccountMeta::new(payer.pubkey(), true),
        AccountMeta::new(new_account.pubkey(), true),
        AccountMeta::new(solana_system_interface::program::ID, false),
    ]);

    let tx = Transaction::new_signed_with_payer(&[ix], Some(&payer.pubkey()), &[
        &payer, &new_account
    ],svm.latest_blockhash());

    let simres = svm.simulate_transaction(tx.clone());
    for log in simres.unwrap().meta.logs {
        println!("{:?}",log);
    }

    svm.send_transaction(tx.clone());
    let player_balance =  svm.get_balance(&payer.pubkey());
    let new_account_res =  svm.get_balance(&new_account.pubkey());

    println!("player_balance {:?}",player_balance);
    println!("new_account_res {:?}",new_account_res);

    assert!(player_balance.is_some());
    assert!(new_account_res.is_some());
}