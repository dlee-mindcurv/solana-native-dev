use borsh::{to_vec, BorshDeserialize};
use litesvm::LiteSVM;
use rent::state::{InstructionCommand, PDAAllocator, Pick, UserPick};
use solana_instruction::{AccountMeta, Instruction};
use solana_keypair::{Keypair, Signer};
use solana_program::native_token::LAMPORTS_PER_SOL;
use solana_pubkey::Pubkey;
use solana_transaction::Transaction;

#[test]
pub fn account_size_for_rent() {
    // define svm
    let mut svm = LiteSVM::new();

    //define a program_id for this program
    let program_id = Pubkey::new_unique();

    // add program to the svm
    svm.add_program_from_file(program_id, "target/deploy/rent.so")
        .unwrap();

    //define an account that will pay
    let payer = Keypair::new();

    // add some lamports to the payer
    svm.airdrop(&payer.pubkey(), LAMPORTS_PER_SOL).unwrap();

    //define an account for PDA (use the payer as aseed)
    let (pda, bump) =
        Pubkey::find_program_address(&[b"rent", &payer.pubkey().as_ref()], &program_id);

    let user_picks = UserPick::new(Pick::England, Pick::Netherlands);

    let ix_command_bytes =
        to_vec(&InstructionCommand::Add(user_picks, PDAAllocator { bump })).unwrap();

    let ix = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new(pda, false),
            AccountMeta::new(solana_system_interface::program::ID, false),
        ],

        data: ix_command_bytes,
    };

    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &[&payer],
        svm.latest_blockhash(),
    );

    let tx_res = svm.send_transaction(tx).unwrap();

    for log in tx_res.logs {
        println!("{:?}", log);
    }

    let account_data = UserPick::try_from_slice(&svm.get_account(&pda).unwrap().data).unwrap();

    assert_eq!(account_data.pick1, Pick::England);
    assert_eq!(account_data.pick2, Pick::Netherlands);
}
