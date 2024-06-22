use alloy::{primitives::Address, signers::local::PrivateKeySigner};
use std::env;

pub struct Config {
    pub rpc_url: String,
    pub private_key_funding: PrivateKeySigner,
    pub address_funding: Address,
    pub private_key_hacked: PrivateKeySigner,
    pub address_hacked: Address,
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

impl Config {
    pub fn new() -> Self {
        dotenv::dotenv().ok();

        let private_key_funding: PrivateKeySigner = env::var("PRIVATE_KEY_FUNDING")
            .expect("PRIVATE_KEY_FUNDING must be set")
            .parse()
            .expect("should parse private key");

        let private_key_hacked: PrivateKeySigner = env::var("PRIVATE_KEY_HACKED")
            .expect("PRIVATE_KEY_HACKED must be set")
            .parse()
            .expect("should parse private key");

        let address_funding = private_key_funding.address();
        let address_hacked = private_key_hacked.address();

        Config {
            rpc_url: env::var("RPC_URL").expect("RPC_URL must be set"),
            private_key_funding,
            address_funding,
            private_key_hacked,
            address_hacked,
        }
    }
}
