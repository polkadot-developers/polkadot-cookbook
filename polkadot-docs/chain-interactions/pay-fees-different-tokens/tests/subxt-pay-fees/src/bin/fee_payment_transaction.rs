use std::str::FromStr;
use subxt::config::{
    Config, DefaultExtrinsicParamsBuilder, DefaultTransactionExtensions, PolkadotConfig,
};
use subxt::utils::AccountId32;
use subxt::{OnlineClient, SubstrateConfig};

#[subxt::subxt(
    runtime_metadata_path = "metadata/asset_hub.scale",
    derive_for_type(
        path = "staging_xcm::v5::location::Location",
        derive = "Clone, Eq, PartialEq, codec::Encode",
        recursive
    )
)]
pub mod asset_hub {}

use asset_hub::runtime_types::staging_xcm::v5::{
    junction::Junction, junctions::Junctions, location::Location,
};

#[derive(Debug, Default, Clone)]
pub struct AssetHubConfig;

impl Config for AssetHubConfig {
    type AccountId = <PolkadotConfig as Config>::AccountId;
    type Address = <PolkadotConfig as Config>::Address;
    type Signature = <PolkadotConfig as Config>::Signature;
    type Hasher = <PolkadotConfig as Config>::Hasher;
    type Header = <SubstrateConfig as Config>::Header;
    type TransactionExtensions = DefaultTransactionExtensions<AssetHubConfig>;
    type AssetId = Location;
}

const POLKADOT_HUB_RPC: &str = "ws://localhost:8000";
const TARGET_ADDRESS: &str = "14E5nqKAp3oAJcmzgZhUD2RcptBeUBScxKHgJKU4HPNcKVf3";
const TRANSFER_AMOUNT: u128 = 3_000_000_000;
const USDT_ASSET_ID: u128 = 1984;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api = OnlineClient::<AssetHubConfig>::from_url(POLKADOT_HUB_RPC).await?;
    println!("Connected to Polkadot Hub (Chopsticks fork)");

    let at_block = api.at_current_block().await?;

    let alice = subxt_signer::sr25519::dev::alice();
    println!("Sender (Alice): {}", AccountId32::from(alice.public_key()));

    let dest = AccountId32::from_str(TARGET_ADDRESS)?;
    let tx = asset_hub::transactions()
        .balances()
        .transfer_keep_alive(dest.into(), TRANSFER_AMOUNT);

    let asset_location = Location {
        parents: 0,
        interior: Junctions::X2([
            Junction::PalletInstance(50),
            Junction::GeneralIndex(USDT_ASSET_ID),
        ]),
    };

    let tx_params = DefaultExtrinsicParamsBuilder::<AssetHubConfig>::new()
        .tip_of(0, asset_location)
        .build();

    println!("Signing and submitting transaction...");
    let progress = at_block
        .transactions()
        .sign_and_submit_then_watch(&tx, &alice, tx_params)
        .await?;

    let in_block = progress.wait_for_finalized().await?;
    let block_hash = in_block.block_hash();
    let events = in_block.wait_for_success().await?;

    println!("Transaction finalized in block: {:?}", block_hash);
    println!("Events:");
    for event in events.iter() {
        let event = event?;
        println!("  {}.{}", event.pallet_name(), event.event_name());
    }

    Ok(())
}
