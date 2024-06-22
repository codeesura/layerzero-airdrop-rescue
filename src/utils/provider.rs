use alloy::providers::{Provider as AlloyProvider, ProviderBuilder};
use eyre::{Report, Result};
use std::sync::Arc;

use super::config;

pub type ConcreteProvider = Arc<dyn AlloyProvider + Send + Sync>;

pub async fn setup() -> Result<ConcreteProvider, Report> {
    let config = config::Config::new();
    let provider =
        ProviderBuilder::new().with_recommended_fillers().on_builtin(&config.rpc_url).await?;

    Ok(Arc::new(provider))
}
