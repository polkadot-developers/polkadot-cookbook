use std::str::FromStr;
use subxt::dynamic::Value;
use subxt::utils::AccountId32;
use subxt::{OnlineClient, PolkadotConfig};

#[subxt::subxt(runtime_metadata_path = "polkadot_testnet_metadata.scale")]
pub mod polkadot_testnet {}

const POLKADOT_TESTNET_RPC: &str = "wss://asset-hub-paseo.dotters.network";
const ACCOUNT_ADDRESS: &str = "5GgbDVeKZwCmMHzn58iFSgSZDTojRMM52arXnuNXto28R7mg";

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api = OnlineClient::<PolkadotConfig>::from_url(POLKADOT_TESTNET_RPC).await?;

    let at_block = api.runtime_api().at_latest().await?;

    println!("Connected to Polkadot Hub TestNet");
    println!("Querying runtime APIs for: {}\n", ACCOUNT_ADDRESS);

    let account = AccountId32::from_str(ACCOUNT_ADDRESS)?;

    // Call AccountNonceApi using static interface
    let nonce_call = polkadot_testnet::apis()
        .account_nonce_api()
        .account_nonce(account);
    let nonce = at_block.call(nonce_call).await?;
    println!("AccountNonceApi Results:");
    println!("  Account Nonce: {}", nonce);

    // Call Metadata API using dynamic interface
    let metadata_versions_call =
        subxt::dynamic::runtime_api_call("Metadata", "metadata_versions", Vec::<Value>::new());
    let versions_result = at_block.call(metadata_versions_call).await?;
    println!("\nMetadata API Results:");
    println!("  Supported Metadata Versions: {:?}", versions_result.to_value()?);

    Ok(())
}
