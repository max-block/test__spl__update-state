use borsh::BorshSerialize;
use solana_program::instruction::{AccountMeta, Instruction};
use solana_program_test::*;
use solana_sdk::{account::Account, pubkey::Pubkey, signature::Signer, transaction::Transaction};
use update_state::{process_instruction, InstructionData};

#[tokio::test]
async fn test_program() {
    println!("bpf test");
    let program_id = Pubkey::new_unique();
    let state_pubkey = Pubkey::new_unique();

    let mut program_test = ProgramTest::new("update_state", program_id, processor!(process_instruction));
    program_test.add_account(state_pubkey, Account { lamports: 7777777777777777777, owner: program_id, ..Account::default() });
    

    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    let ix_data = InstructionData { f1: 777, f2: String::from("foo") }.try_to_vec().unwrap();
    println!("ix_data: {:?}", ix_data);


    // Make tx
    let mut transaction = Transaction::new_with_payer(
        &[Instruction::new_with_bytes(program_id, &ix_data, vec![AccountMeta::new(state_pubkey, false)])],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();
}
