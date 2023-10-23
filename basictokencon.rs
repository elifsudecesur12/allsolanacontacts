use solana_program::{
    account_info::next_account_info,
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use spl_token::{
    instruction::{initialize_account, initialize_mint, mint_to, set_authority, AuthorityType},
    state::{Account, Mint},
};

solana_program::declare_id!("Token1111111111111111111111111111111111111");

#[entrypoint]
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let token_mint_account = next_account_info(accounts_iter)?;

    let new_token_account = next_account_info(accounts_iter)?;
    let new_token_owner = next_account_info(accounts_iter)?;

  
    initialize_mint(
        program_id,
        token_mint_account,
        new_token_owner.key,
        None,
        0,
    )?;

    let mint_pubkey = *token_mint_account.key;
    initialize_account(program_id, new_token_account, &mint_pubkey, new_token_owner.key)?;

    
    if instruction_data.len() != 8 {
        return Err(ProgramError::InvalidInstructionData);
    }

   
    let token_amount = u64::from_le_bytes(instruction_data[0..8].try_into().unwrap());

    mint_to(program_id, &mint_pubkey, new_token_account, new_token_owner, &[&new_token_owner], token_amount)?;

    Ok(())
}
