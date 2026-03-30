from substrateinterface import SubstrateInterface

POLKADOT_TESTNET_RPC = "wss://asset-hub-paseo.dotters.network"
ACCOUNT_ADDRESS = "5GgbDVeKZwCmMHzn58iFSgSZDTojRMM52arXnuNXto28R7mg"


def main():
    substrate = SubstrateInterface(url=POLKADOT_TESTNET_RPC)

    print("Connected to Polkadot Hub TestNet")
    print(f"Querying runtime APIs for: {ACCOUNT_ADDRESS}\n")

    # Call AccountNonceApi to get the account nonce
    nonce = substrate.runtime_call("AccountNonceApi", "account_nonce", [ACCOUNT_ADDRESS])
    print("AccountNonceApi Results:")
    print(f"  Account Nonce: {nonce.value}")

    # Query runtime version using Core runtime API
    version = substrate.runtime_call("Core", "version", [])
    print("\nCore API Results:")
    print(f"  Spec Name: {version.value['spec_name']}")
    print(f"  Spec Version: {version.value['spec_version']}")
    print(f"  Impl Version: {version.value['impl_version']}")

    substrate.close()


if __name__ == "__main__":
    main()
