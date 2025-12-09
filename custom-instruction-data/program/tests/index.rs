use borsh::{to_vec, BorshSerialize};
use custom_instruction_data::state::{ProgramInstruction, Student, University, User};
use litesvm::LiteSVM;
use solana_instruction::{AccountMeta, Instruction};
use solana_keypair::{Keypair, Signer};
use solana_program::native_token::LAMPORTS_PER_SOL;
use solana_pubkey::Pubkey;
use solana_transaction::Transaction;
use uuid::Uuid;

#[test]
pub fn custom_instruction_data() {
    //define the svm
    let mut svm = LiteSVM::new();

    // define a program id
    let program_id = Pubkey::new_unique();

    svm.add_program_from_file(program_id, "target/deploy/custom_instruction_data.so");

    let payer = Keypair::new();

    // get the system program
    let system_program = solana_system_interface::program::ID;

    // assign lamports to the paying account
    svm.airdrop(&payer.pubkey(), LAMPORTS_PER_SOL).unwrap();

    // define a tx to create a User PDA account for a user.
    let (pda_account, bump) = Pubkey::find_program_address(
        &[b"custom_instruction_data", &payer.pubkey().as_ref()],
        &program_id,
    );

    let new_user_data = User {
        name: "David".to_string(),
        age: 23,
    };

    let tx_command = ProgramInstruction::CreateUser(new_user_data);

    let create_user_ix = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new(pda_account, false),
            AccountMeta::new(system_program, false),
        ],
        data: to_vec(&tx_command).unwrap(),
    };

    let student_data = Student {
        student_id: Uuid::new_v4().to_string(),
        university: University::UniversityOfToronto,
        grades: [85, 78, 98, 79, 95],
    };

    let tx_student_command = ProgramInstruction::CreateStudent(student_data);

    let create_student_ix = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new(pda_account, false),
            AccountMeta::new(system_program, false),
        ],
        data: to_vec(&tx_student_command).unwrap(),
    };

    let create_user_tx = Transaction::new_signed_with_payer(
        &[create_user_ix, create_student_ix],
        Some(&payer.pubkey()),
        &[&payer],
        svm.latest_blockhash(),
    );

    let create_user_tx_result = svm.send_transaction(create_user_tx).unwrap().logs;

    for log in create_user_tx_result {
        println!("{:?}", log);
    }
}
