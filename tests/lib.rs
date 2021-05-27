use borsh::{BorshSerialize};
use solana_program::instruction::{AccountMeta, Instruction};
use solana_program_test::*;
use solana_sdk::{account::Account, pubkey::Pubkey, signature::Signer, transaction::Transaction};
use update_state::{process_instruction, State, InstructionData};

#[tokio::test]
async fn test_program() {
    let program_id = Pubkey::new_unique();
    let state_pubkey = Pubkey::new_unique();

    let mut program_test = ProgramTest::new("update_state", program_id, processor!(process_instruction));
    let data = State{counter: 0, f1: 0, f2: String::from("")}.try_to_vec().unwrap();
    program_test.add_account(state_pubkey, Account { lamports: 77777, data, owner: program_id, ..Account::default() });
    

    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    


    // Call program first time
    let init_data = InstructionData { f1: 1, f2: String::from("foo") }.try_to_vec().unwrap();
    let mut transaction = Transaction::new_with_payer(
        &[Instruction::new_with_bytes(program_id, &init_data, vec![AccountMeta::new(state_pubkey, false)])],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();

    // // Call program
    // let state_account = banks_client.get_account(state_pubkey).await;
    // println!("state_account: {:?}", state_account);
}
