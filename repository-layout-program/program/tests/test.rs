use borsh::to_vec;
use litesvm::LiteSVM;
use repository_layout_program::processor::CarnivalInstructionData;
use solana_instruction::{AccountMeta, Instruction};
use solana_keypair::{Keypair, Signer};
use solana_program::native_token::LAMPORTS_PER_SOL;
use solana_pubkey::Pubkey;
use solana_transaction::Transaction;

#[test]
pub fn test_repository_layout_program() {
    // define the svm
    let mut svm = LiteSVM::new();

    // define a program id to add to the svm
    let program_id = Pubkey::new_unique();

    // add program to the svm
    svm.add_program_from_file(program_id, "target/deploy/repository_layout_program.so")
        .unwrap();

    // define a payer to the tx
    let payer = Keypair::new();

    // airdrop amount to payer
    svm.airdrop(&payer.pubkey(), LAMPORTS_PER_SOL).unwrap();

    // define a user (often payer) where the PDA will be seeded to
    let user = Keypair::new();

    // find the pda for the user account
    let (pda, bump) = Pubkey::find_program_address(
        &[b"repository_layout", &user.pubkey().as_ref()],
        &program_id,
    );

    // create instruction to ride
    let ix_ride_data = CarnivalInstructionData {
        name: "Jimmy".to_string(),
        height: 100,
        ticket_count: 10,
        attraction: "ride".to_string(),
        attraction_name: "Scrambler".to_string(),
    };

    // serialize data
    let ix_ride_data = to_vec(&ix_ride_data).unwrap();

    let ix_ride = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new(user.pubkey(), true),
            AccountMeta::new(pda, false),
            AccountMeta::new(solana_system_interface::program::ID, false),
        ],
        data: ix_ride_data,
    };

    let ix_game_data = to_vec(&CarnivalInstructionData {
        name: "Jimmy".to_string(),
        height: 36,
        ticket_count: 15,
        attraction: "game".to_string(),
        attraction_name: "I Got It!".to_string(),
    })
    .unwrap();

    let ix_game = Instruction {
        program_id,
        accounts: vec![AccountMeta::new(payer.pubkey(), true)],
        data: ix_game_data,
    };

    let ix_food_data = to_vec(&CarnivalInstructionData {
        name: "Jimmy".to_string(),
        height: 36,
        ticket_count: 15,
        attraction: "food".to_string(),
        attraction_name: "Taco Shack".to_string(),
    })
    .unwrap();

    let ix_food = Instruction {
        program_id,
        accounts: vec![AccountMeta::new(payer.pubkey(), true)],
        data: ix_food_data,
    };

    let txs = Transaction::new_signed_with_payer(
        &[ix_ride, ix_game, ix_food],
        Some(&payer.pubkey()),
        &[&payer, &user],
        svm.latest_blockhash(),
    );

    let txs_res = svm.send_transaction(txs);

    assert_eq!(txs_res.is_ok(), true);

    for log in txs_res.unwrap().logs {
        println!("{:?}", log)
    }
}
