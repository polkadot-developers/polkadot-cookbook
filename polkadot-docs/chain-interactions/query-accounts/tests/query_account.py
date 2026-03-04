from substrateinterface import SubstrateInterface

WS_ENDPOINT = "wss://asset-hub-paseo.dotters.network"
ACCOUNT_ADDRESS = "5GgbDVeKZwCmMHzn58iFSgSZDTojRMM52arXnuNXto28R7mg"


def main():
    substrate = SubstrateInterface(url=WS_ENDPOINT)
    print("Python Substrate Interface: Connected to Asset Hub Paseo")

    account_info = substrate.query(
        module="System",
        storage_function="Account",
        params=[ACCOUNT_ADDRESS],
    )

    print(f"Python Substrate Interface: Querying account {ACCOUNT_ADDRESS}")
    print(f"  Nonce: {account_info.value['nonce']}")
    print(f"  Consumers: {account_info.value['consumers']}")
    print(f"  Providers: {account_info.value['providers']}")
    print(f"  Sufficients: {account_info.value['sufficients']}")
    print(f"  Free Balance: {account_info.value['data']['free']}")
    print(f"  Reserved Balance: {account_info.value['data']['reserved']}")
    print(f"  Frozen Balance: {account_info.value['data']['frozen']}")

    assert account_info.value["nonce"] is not None
    assert account_info.value["data"]["free"] is not None
    assert account_info.value["data"]["reserved"] is not None
    assert account_info.value["data"]["frozen"] is not None

    substrate.close()
    print("All assertions passed!")


if __name__ == "__main__":
    main()
