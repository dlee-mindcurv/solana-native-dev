use accounts_that_expand_in_size::processor::ReallocInstruction;
use accounts_that_expand_in_size::state::address_info::AddressInfo;
use accounts_that_expand_in_size::state::enhanced_address_info::{
    EnhancedAddressInfo, EnhancedAddressInfoExtender,
};
use borsh::{from_slice, to_vec};
use litesvm::LiteSVM;
use solana_instruction::{AccountMeta, Instruction};
use solana_keypair::{Keypair, Signer};
use solana_program::native_token::LAMPORTS_PER_SOL;
use solana_pubkey::Pubkey;
use solana_transaction::Transaction;

#[test]
pub fn test_accounts_that_expand_in_size() {
    //define a program_id for the progra,
    let program_id = Pubkey::new_unique();

    // define a payer KeyPair
    let payer = Keypair::new();

    // define a user (often the user is the payer)
    let user = Keypair::new();

    // define the test svm
    let mut svm = LiteSVM::new();

    // load the program from the so file
    svm.add_program_from_file(&program_id, "target/deploy/accounts_that_expand_in_size.so");

    // airdrop lamports to the payer account
    svm.airdrop(&payer.pubkey(), LAMPORTS_PER_SOL);

    let address_info = AddressInfo {
        name: "Canada".to_string(),
        city: "Toronto".to_string(),
        street: "123 Main St".to_string(),
        house_number: 123,
    };

    // define create instruction
    let create_command = to_vec(&ReallocInstruction::Create(address_info)).unwrap();

    // define create ix
    let create_ix = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new(user.pubkey(), true),
            // AccountMeta::new(pda, false),
            AccountMeta::new(solana_system_interface::program::ID, false),
        ],
        data: create_command,
    };

    let tx = Transaction::new_signed_with_payer(
        &[create_ix],
        Some(&payer.pubkey()),
        &[&payer, &user],
        svm.latest_blockhash(),
    );

    let create_ix_res = svm.send_transaction(tx);

    for log in create_ix_res.unwrap().logs {
        println!("{:?}", log);
    }

    let created_account = svm.get_account(&user.pubkey()).unwrap();

    let data = from_slice::<AddressInfo>(&created_account.data).unwrap();

    assert_eq!(data.city, "Toronto".to_string());
    assert_eq!(data.name, "Canada".to_string());
    assert_eq!(data.street, "123 Main St".to_string());
    assert_eq!(data.house_number, 123u8);

    // Test the additions to the struct data

    // airdrop new user som lamports
    // svm.airdrop(&user.pubkey(), LAMPORTS_PER_SOL);

    // create an ix to update the User info
    let enhanced__data = EnhancedAddressInfoExtender {
        state: "Ontario".to_string(),
        zip: 123456,
    };
    let update_ix_command = ReallocInstruction::ReallocateWithoutZeroInit(enhanced__data);

    let update_ix_data = to_vec(&update_ix_command).unwrap();

    let ix_enhanced = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new(user.pubkey(), true),
            AccountMeta::new(solana_system_interface::program::ID, false),
        ],
        data: update_ix_data,
    };

    let tx_enhanced = Transaction::new_signed_with_payer(
        &[ix_enhanced],
        Some(&payer.pubkey()),
        &[&payer, &user],
        svm.latest_blockhash(),
    );

    let tx_enhanced_res = svm.send_transaction(tx_enhanced).unwrap();

    for log in tx_enhanced_res.logs {
        println!("{:?}", log);
    }

    let updated_account = svm.get_account(&user.pubkey()).unwrap();

    let data = from_slice::<EnhancedAddressInfo>(&updated_account.data).unwrap();

    assert_eq!(data.city, "Toronto".to_string());
    assert_eq!(data.name, "Canada".to_string());
    assert_eq!(data.street, "123 Main St".to_string());
    assert_eq!(data.state, "Ontario".to_string());
    assert_eq!(data.zip, 123456);
}
