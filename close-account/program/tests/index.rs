use borsh::to_vec;
use close_account::state::{CreateUserArgs, UserInfo};
use litesvm::LiteSVM;
use solana_keypair::{Keypair, Signer};
use solana_sdk::message::{AccountMeta, Instruction};
use solana_sdk::native_token::LAMPORTS_PER_SOL;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::transaction::Transaction;

#[test]
pub fn test_close_option() -> Result<(), Box<dyn std::error::Error>> {
    let svm = &mut LiteSVM::new();
    let blockhash = svm.latest_blockhash();
    let program_id = Pubkey::new_unique();
    let payer = Keypair::new();

    let _ = svm.add_program_from_file(program_id, "target/deploy/close_account.so");
    let _ = svm.airdrop(&payer.pubkey(), LAMPORTS_PER_SOL).unwrap();

    let (test_acount_pda_key, bump) =
        Pubkey::find_program_address(&[b"USERINFO", &payer.pubkey().as_ref()], &program_id);

    let user_info = UserInfo {
        age: 23,
        name: "David".to_string(),
    };

    let create_user_args = CreateUserArgs { user_info, bump };

    let mut discriminator = vec![0];
    let mut vect = to_vec(&create_user_args).unwrap();

    discriminator.append(&mut vect);

    // Instruction to create the user
    let ix = Instruction::new_with_bytes(
        program_id,
        &discriminator,
        vec![
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new(test_acount_pda_key, false),
            AccountMeta::new(solana_system_interface::program::ID, false),
        ],
    );

    let tx = Transaction::new_signed_with_payer(&[ix], Some(&payer.pubkey()), &[&payer], blockhash);

    let txsim = svm.simulate_transaction(tx.clone());
    for logs in txsim.unwrap().meta.logs {
        println!("{}", logs)
    }
    let tx_response = svm.send_transaction(tx.clone());
    assert!(tx_response.is_ok());

    let account_data = Some(svm.get_account(&test_acount_pda_key))
        .unwrap()
        .unwrap()
        .data;

    let deserialized_data = borsh::from_slice::<UserInfo>(&account_data)?;

    assert_eq!(deserialized_data.name, "David", "data not added correctly");
    assert_eq!(deserialized_data.age, 23, "age not added correctly");

    // Instruction to Close the User
    // You can do it this way by passing the Enum or set the data's byte array to &[1u8]
    // let data = to_vec(&CloseUser).unwrap();
    let data = &[1u8];

    let ix = Instruction::new_with_bytes(
        program_id,
        data,
        vec![
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new(test_acount_pda_key, false),
            AccountMeta::new(solana_system_interface::program::ID, false),
        ],
    );

    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &[&payer],
        svm.latest_blockhash(),
    );

    assert!(svm.send_transaction(tx).is_ok());

    let account_data = Some(svm.get_account(&test_acount_pda_key))
        .unwrap()
        .unwrap();

    assert_eq!(
        account_data.owner,
        solana_system_interface::program::ID,
        "account not closed:  account not returned to owner"
    );

    Ok(())
}
