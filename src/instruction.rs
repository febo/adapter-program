use solana_instruction::{AccountMeta, Instruction};
use solana_pubkey::Pubkey;

pub enum AdapterInstruction {
    /// Create a new account.
    ///
    /// This instruction uses the SDK `invoke` instruction to
    /// create a new account.
    CreateAccountV1,

    /// Create a new account.
    ///
    /// This instruction uses the Pinocchio `invoke` instruction to
    /// create a new account.
    CreateAccountV2,
}

pub fn create_account_v1(from: &Pubkey, to: &Pubkey) -> Instruction {
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(*from, true),
            AccountMeta::new(*to, true),
            AccountMeta::new_readonly(solana_system_interface::program::ID, false),
        ],
        data: vec![AdapterInstruction::CreateAccountV1 as u8],
    }
}

pub fn create_account_v2(from: &Pubkey, to: &Pubkey) -> Instruction {
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(*from, true),
            AccountMeta::new(*to, true),
            AccountMeta::new_readonly(solana_system_interface::program::ID, false),
        ],
        data: vec![AdapterInstruction::CreateAccountV2 as u8],
    }
}
