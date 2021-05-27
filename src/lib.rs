use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{account_info::{AccountInfo, next_account_info}, entrypoint, entrypoint::ProgramResult, msg, nonce::state, pubkey::Pubkey};

entrypoint!(process_instruction);

#[derive(BorshSerialize, BorshDeserialize,  Debug)]
pub struct State {
    pub counter: u16,
    pub f1: i32,
    pub f2: String,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct InstructionData {
    pub f1: i32,
    pub f2: String,
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

    

    // let mut zzz = new_state.try_to_vec().unwrap();
    
    // msg!("zzz: {:?}", zzz.as_slice());

    
    // let  data: &mut[u8] = &mut[];
    // *account.data.borrow_mut() =  new_state.try_to_vec().unwrap().as_mut_slice();
    
    

    
    msg!("update-state finish");
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_serialization() {
        let a1 = InstructionData { f1: 777, f2: String::from("foo") }.try_to_vec().unwrap();
        println!("z0: {:?}", a1);
        let ix = InstructionData::try_from_slice(&[9, 3, 0, 0, 3, 0, 0, 0, 102, 111, 111]).unwrap();
        println!("z1: {:?}", ix);
    }
}
