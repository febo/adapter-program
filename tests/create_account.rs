use adapter_program::instruction::{create_account_v1, create_account_v2};
use mollusk_svm::{program::keyed_account_for_system_program, result::Check, Mollusk};
use solana_account::Account;
use solana_pubkey::Pubkey;

pub fn create_funded_account(lamports: u64, owner: Pubkey) -> Account {
    Account {
        lamports,
        owner,
        ..Default::default()
    }
}

#[test]
fn test_create_account_v1() {
    let from_key = Pubkey::new_unique();
    let from_account = create_funded_account(2_000_000_000, solana_system_interface::program::ID);

    let to_key = Pubkey::new_unique();

    let instruction = create_account_v1(&from_key, &to_key);

    let mollusk = Mollusk::new(&adapter_program::ID, "adapter_program");
    mollusk.process_and_validate_instruction(
        &instruction,
        &[
            (from_key, from_account),
            (to_key, Account::default()),
            keyed_account_for_system_program(),
        ],
        &[
            Check::success(),
            // account discriminator
            Check::account(&to_key).space(50).build(),
        ],
    );
}

#[test]
fn test_create_account_v2() {
    let from_key = Pubkey::new_unique();
    let from_account = create_funded_account(2_000_000_000, solana_system_interface::program::ID);

    let to_key = Pubkey::new_unique();

    let instruction = create_account_v2(&from_key, &to_key);

    let mollusk = Mollusk::new(&adapter_program::ID, "adapter_program");
    mollusk.process_and_validate_instruction(
        &instruction,
        &[
            (from_key, from_account),
            (to_key, Account::default()),
            keyed_account_for_system_program(),
        ],
        &[
            Check::success(),
            // account discriminator
            Check::account(&to_key).space(50).build(),
        ],
    );
}
