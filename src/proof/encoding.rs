use crate::api::client::get_proof_and_amount;
use alloy::{
    hex,
    primitives::{Address, Bytes, FixedBytes, U256},
    sol,
    sol_types::SolCall,
};
use eyre::{eyre, Result};
use std::str::FromStr;

sol! {
    #[derive(Debug, PartialEq, Eq)]
    function encodeData(uint256 donationInWei, uint256 amountInWei, address safeWalletAddress, bytes32[] proof) external;
}

pub async fn encode_data(
    hacked_wallet_address: Address,
    safe_wallet_address: Address,
) -> Result<(Bytes, U256)> {
    let proof_data = get_proof_and_amount(hacked_wallet_address.to_checksum(None).as_str())
        .await
        .map_err(|e| eyre!("Error fetching proof data: {:?}", e))?;

    let proof = parse_proof(&proof_data.proof)?;
    let amount_in_wei = U256::from_str(&proof_data.amount)?;
    let donation_in_wei = amount_in_wei / U256::from(35500);

    let encoded = create_encoded_data(donation_in_wei, amount_in_wei, safe_wallet_address, &proof)?;
    let modified_encoded = modify_encoded_data(encoded, proof.len(), amount_in_wei)?;

    let data = format!(
        "0xac6ae3ee0000000000000000000000000000000000000000000000000000000000000002{}",
        modified_encoded
    );
    let bytes_data = Bytes::from(hex::decode(data.strip_prefix("0x").unwrap_or(&data))?);

    Ok((bytes_data, donation_in_wei))
}

fn parse_proof(proof_str: &str) -> Result<Vec<FixedBytes<32>>> {
    proof_str
        .split('|')
        .map(|p| {
            let p = p.strip_prefix("0x").unwrap_or(p);
            let bytes = hex::decode(p)?;
            let mut fixed_bytes = [0u8; 32];
            fixed_bytes.copy_from_slice(&bytes);
            Ok(FixedBytes::from(fixed_bytes))
        })
        .collect()
}

fn create_encoded_data(
    donation_in_wei: U256,
    amount_in_wei: U256,
    safe_wallet_address: Address,
    proof: &[FixedBytes<32>],
) -> Result<String> {
    let encoded = encodeDataCall {
        donationInWei: donation_in_wei,
        amountInWei: amount_in_wei,
        safeWalletAddress: safe_wallet_address,
        proof: proof.to_vec(),
    }
    .abi_encode();

    Ok(hex::encode(&encoded[4..]))
}

fn modify_encoded_data(encoded: String, proof_len: usize, amount_in_wei: U256) -> Result<String> {
    let hex_len = format!("{:x}", proof_len);
    let mut modified = encoded.replace(
        &format!("000000000000000000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000000000000000000000{}", hex_len),
        &format!("000000000000000000000000000000000000000000000000000000000000038000000000000000000000000000000000000000000000000000000000000000{}", hex_len),
    );
    modified += "0000000000000000000000000000000000000000000000000000000000000000";

    let hex_amount_in_wei = format!("{:x}", amount_in_wei);
    modified = modified.replace(
        &format!("{}000000000000000000000000", hex_amount_in_wei),
        &format!("{}00000000000000000000000000000000000000000000000000000000000000c0000000000000000000000000", hex_amount_in_wei),
    );

    Ok(modified)
}
