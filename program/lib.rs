use borsh::{BorshDeserialize, BorshSerialize};
use shank::{ShankAccount, ShankInstruction};
use solana_program::declare_id;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    instruction::AccountMeta,
    instruction::Instruction,
    msg,
    program::invoke,
    program::invoke_signed,
    program_error::ProgramError,
    pubkey::Pubkey,
};

declare_id!("9pRuFihkuA5wzP75xWDoLuLpBhehANoZLrGrySNQRD7T");

/// Define the type of state stored in accounts
#[derive(BorshSerialize, BorshDeserialize, ShankAccount, Debug)]
pub struct GreetingAccount {
    /// number of greetings
    pub counter: u32,
}

/// Define the instructions enum
#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, ShankInstruction, Debug, Clone)]
pub enum Cmd {
    #[account(
        0,
        signer,
        name = "incr",
        desc = "Increment account greeting counter by 1"
    )]
    Nop,
    Incr,
    Message,
    Memo,
}

// Declare and export the program's entrypoint
entrypoint!(main);

// Program entrypoint's implementation
pub fn main(
    program_id: &Pubkey, // Public key of the account the hello world program was loaded into
    accounts: &[AccountInfo], // The account to say hello to
    instruction_data: &[u8], // Ignored, all helloworld instructions are hellos
) -> ProgramResult {
    if instruction_data.len() == 0 {
        return Err(ProgramError::InvalidInstructionData);
    }

    if instruction_data[0] == 0 {
        return nop(program_id, accounts, instruction_data);
    }

    if instruction_data[0] == 1 {
        return incr(program_id, accounts, instruction_data);
    }

    if instruction_data[0] == 2 {
        return message(program_id, accounts, &instruction_data[1..]);
    }

    if instruction_data[0] == 3 {
        return memo(program_id, accounts, &instruction_data[1..]);
    }

    incr(program_id, accounts, instruction_data)
}

pub fn incr(
    program_id: &Pubkey, // Public key of the account the hello world program was loaded into
    accounts: &[AccountInfo], // The account to say hello to
    instruction_data: &[u8], // Ignored, all helloworld instructions are hellos
) -> ProgramResult {
    msg!("Solana starter program entrypoint");

    // Iterating accounts is safer than indexing
    let accounts_iter = &mut accounts.iter();

    // Get the account to say hello to
    let account = next_account_info(accounts_iter)?;

    // The account must be owned by the program in order to modify its data
    if account.owner != program_id {
        msg!("Greeted account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }

    // Increment and store the number of times the account has been greeted
    let mut greeting_account = GreetingAccount::try_from_slice(&account.data.borrow())?;
    greeting_account.counter += 1;
    greeting_account.serialize(&mut &mut account.data.borrow_mut()[..])?;

    msg!("Greeted {} time(s)!", greeting_account.counter);

    let memo = "btwiuse";
    msg!("Memo (len {}): {:?}", memo.len(), memo);

    Ok(())
}

pub fn message(
    program_id: &Pubkey, // Public key of the account the hello world program was loaded into
    accounts: &[AccountInfo], // The account to say hello to
    input: &[u8],        // Ignored, all helloworld instructions are hellos
) -> ProgramResult {
    msg!("Your input is {:?}", &input);
    Ok(())
}

// https://github.com/zoddn/zod/blob/5f2a55d1fce39e8ad17f160252e56343e24cc71e/common/src/memo.rs
// https://github.com/jackrieck/betbook/blob/baa3b68f9d7d9a5432f1c4f9dec3df1ebe9edab9/betbook1/programs/betbook1/src/lib.rs#L99
pub fn build_memo(memo: &[u8], signer_pubkeys: &[&Pubkey]) -> Instruction {
    Instruction {
        program_id: Pubkey::try_from("MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr").unwrap(),
        accounts: signer_pubkeys
            .iter()
            .map(|&pubkey| AccountMeta::new_readonly(*pubkey, true))
            .collect(),
        data: memo.to_vec(),
    }
}

pub fn memo(
    program_id: &Pubkey, // Public key of the account the hello world program was loaded into
    accounts: &[AccountInfo], // The account to say hello to
    instruction_data: &[u8], // Ignored, all helloworld instructions are hellos
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let memo_program = next_account_info(account_info_iter)?;
    invoke(&build_memo(instruction_data, &[]), &[memo_program.clone()])?;
    // invoke_signed(&build_memo(instruction_data, &[]), &[], &[&[b"abc"]])?;
    Ok(())
}

pub fn nop(
    program_id: &Pubkey, // Public key of the account the hello world program was loaded into
    accounts: &[AccountInfo], // The account to say hello to
    instruction_data: &[u8], // Ignored, all helloworld instructions are hellos
) -> ProgramResult {
    Ok(())
}

// Sanity tests
#[cfg(test)]
mod test {
    use super::*;
    use solana_program::clock::Epoch;
    use std::mem;

    #[test]
    fn test_sanity() {
        let program_id = Pubkey::default();
        let key = Pubkey::default();
        let mut lamports = 0;
        let mut data = vec![0; mem::size_of::<u32>()];
        let owner = Pubkey::default();
        let account = AccountInfo::new(
            &key,
            false,
            true,
            &mut lamports,
            &mut data,
            &owner,
            false,
            Epoch::default(),
        );
        let instruction_data: Vec<u8> = Vec::new();

        let accounts = vec![account];

        assert_eq!(
            GreetingAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            0
        );
        process_instruction(&program_id, &accounts, &instruction_data).unwrap();
        assert_eq!(
            GreetingAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            1
        );
        process_instruction(&program_id, &accounts, &instruction_data).unwrap();
        assert_eq!(
            GreetingAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            2
        );
    }
}
