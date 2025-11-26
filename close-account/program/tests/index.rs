use borsh::{to_vec, BorshSerialize};
use close_account::process_program;
use close_account::state::UserInfo;
use litesvm::LiteSVM;
use solana_program::config::program;
use solana_program::example_mocks::solana_sdk::system_instruction::SystemInstruction;
use solana_sdk::message::{AccountMeta, Instruction};
use solana_sdk::native_token::LAMPORTS_PER_SOL;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::transaction::Transaction;

#[test]
pub fn test() {
    let svm = &mut LiteSVM::new();
    let blockhash = svm.latest_blockhash();
    let program_id = Pubkey::new_unique();
    let payer = Keypair::new();

    let _ = svm.add_program_from_file(program_id, "target/deploy/close_account.so");
    let _ = svm.airdrop(&payer.pubkey(), LAMPORTS_PER_SOL).unwrap();

    let (pda, bump) =
        Pubkey::find_program_address(&[b"close_account", &payer.pubkey().as_ref()], &program_id);

    let user_info = UserInfo {
        age: 23,
        name: "David".to_string(),
    };

    let ix = Instruction::new_with_borsh(
        program_id,
        &user_info,
        vec![
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new(pda, false),
            AccountMeta::new(program::ID, false),
        ],
    );

    let tx = Transaction::new_signed_with_payer(&[ix], Some(&payer.pubkey()), &[payer], blockhash);

    let tx_response = svm.send_transaction(tx);
    assert!(tx_response.is_ok());
}
