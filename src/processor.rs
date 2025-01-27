use pinocchio_adapter::{account_info::AccountInfoAdapter, to_program_error};
use pinocchio_system::instructions::CreateAccount;
use solana_account_info::AccountInfo;
use solana_cpi::invoke;
use solana_program_entrypoint::ProgramResult;
use solana_program_error::ProgramError;
use solana_pubkey::Pubkey;
use solana_system_interface::instruction::create_account;

pub(crate) fn process_create_account_v1(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let [from_info, to_info, _system_info] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    invoke(
        &create_account(
            from_info.key,
            to_info.key,
            1_000_000_000,
            50,
            &solana_system_interface::program::ID,
        ),
        &[from_info.clone(), to_info.clone()],
    )
}

pub(crate) fn process_create_account_v2(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let [from_info, to_info, _system_info] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    let (from_adapter, to_adapter) = unsafe {
        (
            AccountInfoAdapter::new_unchecked(from_info),
            AccountInfoAdapter::new_unchecked(to_info),
        )
    };

    CreateAccount {
        from: &from_adapter,
        to: &to_adapter,
        lamports: 1_000_000_000,
        space: 50,
        owner: &pinocchio_system::ID,
    }
    .invoke()
    .map_err(|error| to_program_error!(error))
}
