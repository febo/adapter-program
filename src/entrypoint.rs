use solana_account_info::AccountInfo;
use solana_program_entrypoint::ProgramResult;
use solana_program_error::ProgramError;
use solana_pubkey::Pubkey;

use super::processor::{process_create_account_v1, process_create_account_v2};

pub(crate) fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    match instruction_data.first() {
        Some(&0) => process_create_account_v1(program_id, accounts),
        Some(&1) => process_create_account_v2(program_id, accounts),
        _ => Err(ProgramError::InvalidInstructionData),
    }
}
