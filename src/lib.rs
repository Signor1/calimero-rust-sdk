use calimero_sdk::app;
use calimero_sdk::borsh::{BorshDeserialize, BorshSerialize};

#[app::state]
#[derive(Default, BorshSerialize, BorshDeserialize)]
#[borsh(crate = "calimero_sdk::borsh")]
struct KvStore {}

#[app::logic]
impl KvStore {}
