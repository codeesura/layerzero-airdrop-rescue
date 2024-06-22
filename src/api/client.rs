use crate::utils::constants::API_URL;
use reqwest::{Client, Error};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ProofData {
    pub proof: String,
    pub amount: String,
}

pub async fn get_proof_and_amount(address: &str) -> Result<ProofData, Error> {
    let client = Client::new();
    let url = format!("{}{}", API_URL, address);
    let response = client.get(&url).send().await?;
    response.error_for_status_ref()?;
    let proof_data = response.json::<ProofData>().await?;
    Ok(proof_data)
}
