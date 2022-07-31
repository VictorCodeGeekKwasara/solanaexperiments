use solana_program::{
entrypoint,
msg,
account_info::{next_account_info,AccountInfo},
entrypoint::ProgramResult,
pubkey::Pubkey,
};

entrypoint!(process_instructions);

fn process_instructions(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instructions:&[u8]
)->ProgramResult {

    msg!("pinged");

    Ok(())
}
