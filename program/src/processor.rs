use crate::{
    error::ErrorThingy,
    instruction::{InstructionThingArgs, InstructionThingy},
    state::AccountThingy,
};
use borsh::BorshDeserialize;
use mpl_utils::{assert_derivation, create_or_allocate_account_raw};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_memory::sol_memcpy,
    pubkey::Pubkey,
};

pub struct Processor;
impl Processor {
    pub fn process_instruction(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction: InstructionThingy = InstructionThingy::try_from_slice(instruction_data)?;
        match instruction {
            InstructionThingy::InstructionThing(args) => {
                process_instruction_thing(program_id, accounts, args)
            }
        }
    }
}

fn process_instruction_thing(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: InstructionThingArgs,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let payer = next_account_info(account_info_iter)?;
    let writable_account = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;

    let bump = assert_derivation(
        program_id,
        writable_account,
        &[
            "test".as_bytes(),
            payer.key.as_ref(),
            &args.value.to_ne_bytes(),
        ],
        ErrorThingy::ErrorName,
    )?;

    let seeds = &[
        "test".as_bytes(),
        payer.key.as_ref(),
        &args.value.to_ne_bytes(),
        &[bump],
    ];

    create_or_allocate_account_raw(
        *program_id,
        writable_account,
        system_program,
        payer,
        100,
        seeds,
    )?;

    let data = AccountThingy { thing: args.value };

    let serialized_data = rkyv::to_bytes::<AccountThingy, 100>(&data).unwrap();

    sol_memcpy(
        &mut **writable_account
            .try_borrow_mut_data()
            .map_err(|_| ErrorThingy::ErrorName)?,
        &serialized_data.as_slice(),
        serialized_data.len(),
    );

    Ok(())
}
