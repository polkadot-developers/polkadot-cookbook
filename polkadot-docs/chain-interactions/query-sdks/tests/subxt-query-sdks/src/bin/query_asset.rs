use std::str::FromStr;
use subxt::utils::AccountId32;
use subxt::{OnlineClient, PolkadotConfig};

#[subxt::subxt(runtime_metadata_path = "asset_hub_metadata.scale")]
pub mod asset_hub {}

const ASSET_HUB_RPC: &str = "wss://asset-hub-paseo.dotters.network";
const USDT_ASSET_ID: u32 = 1984;
const USDT_HOLDER_ADDRESS: &str = "13rxtPcR9nsAMzLKJsj6UevMR9TzGmyRohJVgQ6U6K2xeqU3";

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api = OnlineClient::<PolkadotConfig>::from_url(ASSET_HUB_RPC).await?;
    println!("Subxt: Connected to Asset Hub Paseo");

    // Query asset metadata
    let metadata_query = asset_hub::storage().assets().metadata(USDT_ASSET_ID);
    let metadata = api
        .storage()
        .at_latest()
        .await?
        .fetch(&metadata_query)
        .await?;

    if let Some(meta) = metadata {
        println!(
            "  Asset Name: {}",
            String::from_utf8_lossy(&meta.name.0)
        );
        println!(
            "  Asset Symbol: {}",
            String::from_utf8_lossy(&meta.symbol.0)
        );
        println!("  Decimals: {}", meta.decimals);
    } else {
        return Err("Asset metadata not found".into());
    }

    // Query asset details
    let asset_query = asset_hub::storage().assets().asset(USDT_ASSET_ID);
    let asset_details = api
        .storage()
        .at_latest()
        .await?
        .fetch(&asset_query)
        .await?;

    if let Some(details) = asset_details {
        println!("  Asset Owner: {:?}", details.owner);
        println!("  Asset Supply: {}", details.supply);
    } else {
        return Err("Asset details not found".into());
    }

    // Query asset account balance
    let holder = AccountId32::from_str(USDT_HOLDER_ADDRESS)?;
    let account_query = asset_hub::storage().assets().account(USDT_ASSET_ID, holder);
    let asset_account = api
        .storage()
        .at_latest()
        .await?
        .fetch(&account_query)
        .await?;

    if let Some(account) = asset_account {
        println!("  Asset Balance: {}", account.balance);
    } else {
        return Err("Asset account not found".into());
    }

    Ok(())
}
