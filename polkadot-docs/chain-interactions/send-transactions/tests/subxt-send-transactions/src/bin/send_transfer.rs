use std::env;
use std::str::FromStr;
use subxt::utils::AccountId32;
use subxt::{OnlineClient, PolkadotConfig};
use subxt_signer::sr25519::Keypair;
use subxt_signer::SecretUri;

#[subxt::subxt(runtime_metadata_path = "asset_hub_metadata.scale")]
pub mod asset_hub {}

const ASSET_HUB_RPC: &str = "wss://asset-hub-paseo.dotters.network";

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sender_mnemonic = env::var("SENDER_MNEMONIC").unwrap_or_default();
    if sender_mnemonic.is_empty() {
        println!("SENDER_MNEMONIC not set, skipping");
        return Ok(());
    }

    let dest_address = env::var("DEST_ADDRESS")
        .unwrap_or_else(|_| "5GgbDVeKZwCmMHzn58iFSgSZDTojRMM52arXnuNXto28R7mg".to_string());

    let api = OnlineClient::<PolkadotConfig>::from_url(ASSET_HUB_RPC).await?;
    println!("Subxt: Connected to Asset Hub Paseo");

    let uri = SecretUri::from_str(&sender_mnemonic)?;
    let keypair = Keypair::from_uri(&uri)?;

    let dest = AccountId32::from_str(&dest_address)?;
    let dest_multi = subxt::utils::MultiAddress::Id(dest);

    let tx = asset_hub::tx()
        .balances()
        .transfer_keep_alive(dest_multi, 1000);

    let tx_hash = api
        .tx()
        .sign_and_submit_default(&tx, &keypair)
        .await?;

    println!("  Transaction submitted");
    println!("  Block hash: {:?}", tx_hash);

    Ok(())
}
