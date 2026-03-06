from substrateinterface import SubstrateInterface

WS_ENDPOINT = "wss://asset-hub-paseo.dotters.network"
USDT_ASSET_ID = 1984
USDT_HOLDER_ADDRESS = "13rxtPcR9nsAMzLKJsj6UevMR9TzGmyRohJVgQ6U6K2xeqU3"


def main():
    substrate = SubstrateInterface(url=WS_ENDPOINT)
    print("Python Substrate Interface: Connected to Asset Hub Paseo")

    # Query asset metadata
    asset_metadata = substrate.query(
        module="Assets",
        storage_function="Metadata",
        params=[USDT_ASSET_ID],
    )

    print(f"Querying asset metadata for asset ID {USDT_ASSET_ID}")
    print(f"  Asset Name: {asset_metadata.value['name']}")
    print(f"  Asset Symbol: {asset_metadata.value['symbol']}")
    print(f"  Decimals: {asset_metadata.value['decimals']}")

    assert asset_metadata.value["name"] is not None
    assert asset_metadata.value["symbol"] is not None

    # Query asset details
    asset_details = substrate.query(
        module="Assets",
        storage_function="Asset",
        params=[USDT_ASSET_ID],
    )

    print(f"Querying asset details for asset ID {USDT_ASSET_ID}")
    print(f"  Asset Owner: {asset_details.value['owner']}")
    print(f"  Asset Supply: {asset_details.value['supply']}")

    assert asset_details.value["owner"] is not None
    assert asset_details.value["supply"] is not None

    # Query asset account balance
    asset_account = substrate.query(
        module="Assets",
        storage_function="Account",
        params=[USDT_ASSET_ID, USDT_HOLDER_ADDRESS],
    )

    print(f"Querying asset account for {USDT_HOLDER_ADDRESS}")
    print(f"  Asset Balance: {asset_account.value['balance']}")

    assert asset_account.value["balance"] is not None

    substrate.close()
    print("All assertions passed!")


if __name__ == "__main__":
    main()
