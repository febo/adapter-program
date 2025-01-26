use mollusk_svm::Mollusk;
use solana_account::Account;
use solana_pubkey::Pubkey;

pub fn create_funded_account(lamports: u64, owner: Pubkey) -> Account {
    Account {
        lamports,
        owner,
        ..Default::default()
    }
}

/// Create a new Mollusk instance for the given program ID and name.
pub fn setup() -> Mollusk {
    std::env::set_var("SBF_OUT_DIR", "target/deploy");
    Mollusk::new(&adapter_program::ID, "adapter_program")
}
