#![feature(test)]

extern crate test;

mod setup;
use setup::*;

use adapter_program::instruction::{create_account_v1, create_account_v2};
use mollusk_svm::program::keyed_account_for_system_program;
use mollusk_svm_bencher::MolluskComputeUnitBencher;
use solana_pubkey::Pubkey;
use test::Bencher;

#[cfg(test)]
mod create_account {

    use solana_account::Account;

    use super::*;

    #[bench]
    fn run(_bencher: &mut Bencher) {
        let mollusk = setup();
        let mut bencher = MolluskComputeUnitBencher::new(mollusk)
            .must_pass(true)
            .out_dir("target/benches");

        // CreateAccountV1

        let from_key = Pubkey::new_unique();
        let from_account =
            create_funded_account(2_000_000_000, solana_system_interface::program::ID);

        let to_key = Pubkey::new_unique();

        let instruction = create_account_v1(&from_key, &to_key);
        let accounts = [
            (from_key, from_account),
            (to_key, Account::default()),
            keyed_account_for_system_program(),
        ];

        bencher = bencher.bench(("create_account_v1", &instruction, &accounts));

        // CreateAccountV2

        let from_key = Pubkey::new_unique();
        let from_account =
            create_funded_account(2_000_000_000, solana_system_interface::program::ID);

        let to_key = Pubkey::new_unique();

        let instruction = create_account_v2(&from_key, &to_key);
        let accounts = [
            (from_key, from_account),
            (to_key, Account::default()),
            keyed_account_for_system_program(),
        ];

        bencher = bencher.bench(("create_account_v2", &instruction, &accounts));

        // Run the benchmarks.

        bencher.execute();
    }
}
