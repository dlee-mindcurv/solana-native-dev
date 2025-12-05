use borsh::BorshDeserialize;
use borsh::{from_slice, to_vec};
use lever::{PowerStatus, SetPowerStatus};
use litesvm::LiteSVM;
use solana_sdk::message::{AccountMeta, Instruction};
use solana_sdk::native_token::LAMPORTS_PER_SOL;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{Keypair, Signer};
use solana_sdk::transaction::Transaction;

#[test]
pub fn test_hand() {
    // define payer for the transaction
    let payer = Keypair::new();

    // define account hold data for the lever program
    // This will be used to update the change in the SetPowerStatus test
    let power_account = Keypair::new();

    // define the test hand programId for the svm test
    let program_id_hand = Pubkey::new_unique();

    // define the test lever programId for the svm test
    let program_id_lever = Pubkey::new_unique();

    // define the svm
    let mut svm = LiteSVM::new();

    // add hand program
    svm.add_program_from_file(program_id_hand, "target/deploy/hand.so")
        .unwrap();

    // add the lever program
    svm.add_program_from_file(program_id_lever, "../lever/target/deploy/lever.so")
        .unwrap();

    // airdrop some fund to the payer
    svm.airdrop(&payer.pubkey(), LAMPORTS_PER_SOL).unwrap();

    // create the borsh DTO
    let power_status_data = PowerStatus { is_on: true };

    // 1.  create the instruction to INITIALIZE the LEVER program
    // If the Struct on the client matches that on-program chain's "try_from_bytes"
    // This is similar as to how try_from_bytes works with ENUM's except the "condition" is byte rather than the struct type
    let initiate_ix = Instruction {
        program_id: program_id_lever,
        accounts: vec![
            AccountMeta::new(payer.pubkey(), true),         // payer
            AccountMeta::new(power_account.pubkey(), true), // power instruction
            AccountMeta::new(solana_system_interface::program::ID, false), // solana program // unused
        ],
        data: to_vec(&power_status_data).unwrap(),
    };

    // 1. INITIALZE THE LEVER PROGRAM with a value
    // because we are not using PDA for this we need to send signed tx with both payer and new account
    let tx = Transaction::new_signed_with_payer(
        &[initiate_ix],
        Some(&payer.pubkey()),
        &[&payer, &power_account],
        svm.latest_blockhash(),
    );

    let meta = svm.simulate_transaction(tx.clone()).unwrap().meta;

    for log in meta.logs {
        println!("{:?}", log)
    }

    svm.send_transaction(tx.clone()).unwrap();

    // check value at adderess
    let account_data_is_on = from_slice::<PowerStatus>(
        svm.get_account(&power_account.pubkey())
            .unwrap()
            .data
            .as_ref(),
    )
    .unwrap()
    .is_on;

    assert!(account_data_is_on);

    // 2. Change the staus of the LEVER through the use of the "Hand"

    // create the instruction
    let hand_ix = Instruction {
        program_id: program_id_hand,
        accounts: vec![
            AccountMeta::new(power_account.pubkey(), false),
            AccountMeta::new(program_id_lever, false),
        ],
        data: to_vec(&SetPowerStatus {
            name: "David Lee".to_string(),
        })
        .unwrap(),
    };

    let hand_ix2 = Instruction {
        program_id: program_id_hand,
        accounts: vec![
            AccountMeta::new(power_account.pubkey(), false),
            AccountMeta::new(program_id_lever, false),
        ],
        data: to_vec(&SetPowerStatus {
            name: "Christine Lee".to_string(),
        })
        .unwrap(),
    };
    let hand_ix3 = Instruction {
        program_id: program_id_hand,
        accounts: vec![
            AccountMeta::new(power_account.pubkey(), false),
            AccountMeta::new(program_id_lever, false),
        ],
        data: to_vec(&SetPowerStatus {
            name: "Marty Lee".to_string(),
        })
        .unwrap(),
    };

    let tx_hand = Transaction::new_signed_with_payer(
        &[hand_ix, hand_ix2, hand_ix3],
        Some(&payer.pubkey()),
        &[&payer],
        svm.latest_blockhash(),
    );

    let meta = svm.simulate_transaction(tx_hand.clone()).unwrap().meta;

    for log in meta.logs {
        println!("{:?}", log)
    }

    svm.send_transaction(tx_hand.clone()).unwrap();

    let is_power_on = PowerStatus::try_from_slice(
        svm.get_account(&power_account.pubkey())
            .unwrap()
            .data
            .as_ref(),
    )
    .unwrap();

    assert_eq!(is_power_on.is_on, false)
}
