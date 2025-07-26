use borsh::{BorshDeserialize,BorshSerialize};
use solana_program::{account_info::{next_account_info, AccountInfo},
    entrypoint::{ ProgramResult},
    entrypoint,
    msg,
    pubkey::Pubkey
};

#[derive(BorshDeserialize,BorshSerialize)]
struct Counter{
    count:u32
}

#[derive(BorshDeserialize,BorshSerialize)]
enum InstructionType{
    Increment(u32),
    Decrement(u32)
}

entrypoint!(counter_contract);

pub fn counter_contract(
    program_id:&Pubkey,
    accounts:&[AccountInfo],
    instruction_data:&[u8]
)->ProgramResult{
    let acc=next_account_info(&mut accounts.iter())?;

    let instruction_type_data=InstructionType::try_from_slice(instruction_data)?;
    let mut count_data=Counter::try_from_slice(&acc.data.borrow_mut())?;

    match instruction_type_data {
        InstructionType::Increment(value)=>{
            msg!("Executing increase");
            count_data.count+=value;
        },
        InstructionType::Decrement(value)=>{
            msg!("Executing decrease");
            count_data.count-=value;
        }
    }  

    count_data.serialize(&mut *acc.data.borrow_mut()); 

    msg!("Counter updated ");
    Ok(())

}