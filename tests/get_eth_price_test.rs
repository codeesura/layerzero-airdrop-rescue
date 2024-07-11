use eyre::Result;
use layerzero_airdrop_rescue::proof::encoding::get_eth_price;

#[tokio::test]
async fn test_get_eth_price() -> Result<()> {
    let eth_price = get_eth_price().await?;
    println!("ETH price in cents: {}", eth_price);
    assert!(eth_price > 0, "ETH price should be greater than 0");
    Ok(())
}
