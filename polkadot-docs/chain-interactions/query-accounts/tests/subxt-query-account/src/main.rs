use std::str::FromStr;
use subxt::utils::AccountId32;
use subxt::{OnlineClient, PolkadotConfig};

#[subxt::subxt(runtime_metadata_path = "polkadot_testnet_metadata.scale")]
pub mod polkadot_testnet {}

const POLKADOT_TESTNET_RPC: &str = "wss://asset-hub-paseo.dotters.network";
const ACCOUNT_ADDRESS: &str = "5GgbDVeKZwCmMHzn58iFSgSZDTojRMM52arXnuNXto28R7mg";

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api = OnlineClient::<PolkadotConfig>::from_url(POLKADOT_TESTNET_RPC).await?;
    println!("Subxt: Connected to Asset Hub Paseo");

    let account = AccountId32::from_str(ACCOUNT_ADDRESS)?;
    println!("\nSubxt: Querying account: {}\n", account);

    let storage_query = polkadot_testnet::storage().system().account(account);
    let account_info = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;

    if let Some(info) = account_info {
        println!("  Nonce: {}", info.nonce);
        println!("  Consumers: {}", info.consumers);
        println!("  Providers: {}", info.providers);
        println!("  Sufficients: {}", info.sufficients);
        println!("  Free Balance: {}", info.data.free);
        println!("  Reserved Balance: {}", info.data.reserved);
        println!("  Frozen Balance: {}", info.data.frozen);
    } else {
        return Err("Account not found".into());
    }

    Ok(())
}
