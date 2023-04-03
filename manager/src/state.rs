// use schemars::JsonSchema;
// use secret_toolkit::storage::Keymap;
// use serde::{Deserialize, Serialize};

// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
// pub struct State {
//     pub count: i32,
//     pub address: String,
// }

pub const EXECUTE_INCREMENT_REPLY_ID: u64 = 1;
// pub static CONTRACTS: Keymap<String, String> = Keymap::new(b"contracts");

/// pad handle responses and log attributes to blocks of 256 bytes to prevent leaking info based on
/// response size
pub const BLOCK_SIZE: usize = 256;
