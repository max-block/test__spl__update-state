use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

entrypoint!(process_instruction);

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct State {
    pub counter: u16,
    pub f1: i32,
    pub f2: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct InstructionData {
    pub f1: i32,
    pub f2: u64,
}

pub fn process_instruction(_program_id: &Pubkey, accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
    msg!("update-state start: {:?}", instruction_data);
    let account_info_iter = &mut accounts.iter();
    let account = next_account_info(account_info_iter)?;

    msg!("account: {:?}", account);
    let instruction = InstructionData::try_from_slice(instruction_data)?;
    msg!("instruction: {:?}", instruction);

    let mut state = State::try_from_slice(&account.data.borrow())?;
    msg!("prev state: {:?}", state);

    state.counter += 1;
    state.f1 = instruction.f1;
    state.f2 = instruction.f2;
    state.serialize(&mut &mut account.data.borrow_mut()[..]);
    msg!("new state: {:?}", state);

    msg!("data: {:?}", &account.data);

    msg!("update-state finish");
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_serialization() {
        let a1 = InstructionData { f1: 777, f2: 999 }.try_to_vec().unwrap();
        println!("z0: {:?}", a1);
        let ix = InstructionData::try_from_slice(&[9, 3, 0, 0, 231, 3, 0, 0, 0, 0, 0, 0]).unwrap();
        println!("z1: {:?}", ix);

        let s0 = State { counter: 1, f1: 17, f2: 108 }.try_to_vec().unwrap();
        println!("s0: {:?}", s0);
        let s1 = State::try_from_slice(&[1, 0, 17, 0, 0, 0, 108, 0, 0, 0, 0, 0, 0, 0]);
        println!("s1: {:?}", s1);
    }
}
