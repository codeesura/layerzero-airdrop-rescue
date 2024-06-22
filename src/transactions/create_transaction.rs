use crate::utils::constants::{CHAIN_ID, GAS_LIMIT_CLAIM, GAS_LIMIT_TRANSFER};
use alloy::{
    consensus::TxEnvelope,
    network::{EthereumWallet, TransactionBuilder},
    primitives::{Address, Bytes, U256},
    rpc::types::TransactionRequest,
    signers::local::PrivateKeySigner,
};
use eyre::Result;

pub enum TransactionType {
    Claim,
    Transfer,
}

pub struct TransactionParams {
    pub signer: PrivateKeySigner,
    pub to: Address,
    pub input: Option<Bytes>,
    pub value: U256,
    pub nonce: u64,
    pub gas_price: u128,
    pub transaction_type: TransactionType,
}

pub async fn create_transaction(params: TransactionParams) -> Result<TxEnvelope> {
    let (gas_limit, chain_id) = match params.transaction_type {
        TransactionType::Claim => (GAS_LIMIT_CLAIM, CHAIN_ID),
        TransactionType::Transfer => (GAS_LIMIT_TRANSFER, CHAIN_ID),
    };

    let mut tx = TransactionRequest::default()
        .with_to(params.to)
        .with_value(params.value)
        .with_nonce(params.nonce)
        .with_gas_limit(gas_limit)
        .with_gas_price(params.gas_price)
        .with_max_fee_per_gas(params.gas_price * 2)
        .with_max_priority_fee_per_gas(0)
        .with_chain_id(chain_id);

    if let Some(input) = params.input {
        tx = tx.with_input(input);
    }

    Ok(tx.build(&EthereumWallet::from(params.signer)).await?)
}
