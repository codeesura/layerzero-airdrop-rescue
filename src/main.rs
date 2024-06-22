use alloy::primitives::U256;
use eyre::Result;
use std::time::Duration;
use tokio::time::sleep;

mod api;
mod proof;
mod transactions;
mod utils;

use crate::{
    transactions::send_transaction::{send_claim_transaction, send_transfer_transaction},
    utils::{config::Config, constants::GAS_LIMIT_CLAIM, provider},
};

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::new();
    let provider = provider::setup().await?;

    let (encoded_data, donation_in_wei) =
        proof::encoding::encode_data(config.address_hacked, config.address_funding).await?;

    let gas_price = provider.get_gas_price().await?;

    let hacked_nonce = provider.get_transaction_count(config.address_hacked).await?;
    let funding_nonce = provider.get_transaction_count(config.address_funding).await?;

    send_claim_transaction(
        &provider,
        &config,
        encoded_data,
        donation_in_wei,
        gas_price,
        hacked_nonce,
    )
    .await?;

    sleep(Duration::from_secs(2)).await;

    let gas_cost = gas_price * GAS_LIMIT_CLAIM;
    let total_value = donation_in_wei + U256::from(gas_cost);

    send_transfer_transaction(&provider, &config, total_value, gas_price, funding_nonce).await?;

    sleep(Duration::from_secs(10)).await;

    println!("All transactions sent successfully");

    Ok(())
}
