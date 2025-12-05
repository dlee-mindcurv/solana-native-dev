#![feature(allocator_api)]

use borsh::{from_slice, to_vec, BorshSerialize};
use litesvm::LiteSVM;
use pda_rent_payer::state::{CreatePdaInstruction, RentInstruction, User};
use solana_keypair::Keypair;
use solana_program::instruction::{AccountMeta, Instruction};
use solana_program::native_token::LAMPORTS_PER_SOL;
use solana_pubkey::Pubkey;
use solana_signer::Signer;
use solana_transaction::Transaction;
use std::alloc::Allocator;
use std::fmt::{Debug, Display};

#[test]
pub fn test_process_instruction() {
    // define svm
    let mut svm = LiteSVM::new();

    // define program_id
    let program_id = Pubkey::new_unique();

    //load program into svm
    svm.add_program_from_file(program_id, "target/deploy/pda_rent_payer.so")
        .unwrap();

    // define payer or user keypair
    let payer_account = Keypair::new();

    // add some lamports to the payer account
    svm.airdrop(&payer_account.pubkey(), LAMPORTS_PER_SOL)
        .unwrap();

    // define user account for PDA seed verificaton
    // in this case we are setting the user to be the payer.  This allows a

    //find a PDA off chain to use to store data and fund with lamports
    let (pda_account, bump) = Pubkey::find_program_address(
        &[b"pda-rent-payer", &payer_account.pubkey().as_ref()],
        &program_id,
    );

    // define the payload to be used in the  RentInstruction::CreatePDA() enum
    let payload = User {
        name: "David Lee".to_string(),
        age: 23,
    };

    // Define Enum
    let create_data = RentInstruction::CreatePDA(CreatePdaInstruction { bump, payload });

    // Set the serialized command yp create the PDA
    let create_data_command = to_vec(&create_data).unwrap();

    // Create the instruction to send to the program
    let ix = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(payer_account.pubkey(), true),
            AccountMeta::new(pda_account, false),
            AccountMeta::new(solana_system_interface::program::ID, false),
        ],
        data: create_data_command,
    };

    // Define the transaction, as assign the signed addresses
    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer_account.pubkey()),
        &[&payer_account],
        svm.latest_blockhash(),
    );

    let logs = svm.send_transaction(tx).unwrap().logs;

    // get the latest account changes from the chain
    let saved_pda_data =
        from_slice::<User>(svm.get_account(&pda_account).unwrap().data.as_ref()).unwrap();

    print_logs(logs);
    // TEST
    assert_eq!(saved_pda_data.age, 23);
    assert_eq!(saved_pda_data.name, "David Lee");

    //====== Add lamports to the PDA

    // Define lamports to send
    let update_lamport_data = LAMPORTS_PER_SOL / 1000;

    // Define the lamports deposit size to be used command to be used in the commands
    let update_lamports_instruction = to_vec(&RentInstruction::DepositLamports(
        update_lamport_data as usize,
    ))
    .unwrap();

    let ix_update = Instruction {
        program_id,
        accounts: vec![AccountMeta::new(
            payer_account.insecure_clone().pubkey(),
            true,
        )],
        data: update_lamports_instruction,
    };

    let tx_update = Transaction::new_signed_with_payer(
        &[ix_update],
        Some(&payer_account.pubkey()),
        &[&payer_account],
        svm.latest_blockhash(),
    );

    let logs = svm.send_transaction(tx_update).unwrap().logs;
    print_logs(logs);
}

fn print_logs(logs: Vec<String>) {
    for log in logs {
        println!("{:?}", log)
    }
}
