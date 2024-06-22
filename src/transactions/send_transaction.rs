use crate::{
    transactions::create_transaction::{create_transaction, TransactionParams, TransactionType},
    utils::{config::Config, provider::ConcreteProvider},
};
use alloy::primitives::{Address, Bytes, U256};
use eyre::Result;
use std::str::FromStr;
use tokio::task;

pub async fn send_transaction(
    provider: &ConcreteProvider,
    params: TransactionParams,
    repeat: bool,
) -> Result<()> {
    let tx_raw = create_transaction(params).await?;

    if repeat {
        let provider = provider.clone();
        task::spawn(async move {
            loop {
                match provider.send_tx_envelope(tx_raw.clone()).await {
                    Ok(tx_hash) => println!("Transaction sent with hash: {:?}", tx_hash),
                    Err(e) => eprintln!("Error sending transaction: {:?}", e),
                }
            }
        });
    } else {
        let tx_hash = provider.send_tx_envelope(tx_raw).await?.get_receipt().await?;
        println!("Transaction sent with hash: {:?}", tx_hash);
    }

    Ok(())
}

pub async fn send_claim_transaction(
    provider: &ConcreteProvider,
    config: &Config,
    encoded_data: Bytes,
    amount_in_wei: U256,
    gas_price: u128,
    nonce: u64,
) -> Result<()> {
    let signer = config.private_key_hacked.clone();

    let params = TransactionParams {
        signer,
        to: Address::from_str(crate::utils::constants::CONTRACT_ADDRESS)?,
        input: Some(encoded_data),
        value: amount_in_wei,
        nonce,
        gas_price,
        transaction_type: TransactionType::Claim,
    };

    send_transaction(provider, params, true).await
}

pub async fn send_transfer_transaction(
    provider: &ConcreteProvider,
    config: &Config,
    value: U256,
    gas_price: u128,
    nonce: u64,
) -> Result<()> {
    let signer = config.private_key_funding.clone();

    let params = TransactionParams {
        signer,
        to: config.address_hacked,
        input: None,
        value,
        nonce,
        gas_price,
        transaction_type: TransactionType::Transfer,
    };

    send_transaction(provider, params, false).await
}
