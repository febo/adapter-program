mod entrypoint;
pub mod instruction;
pub(crate) mod processor;

use solana_program_entrypoint::entrypoint;
use solana_pubkey::declare_id;

use entrypoint::process_instruction;

declare_id!("Adapter1111111111111111111111111111111111111");

entrypoint!(process_instruction);
