import os
import sys
from substrateinterface import SubstrateInterface, Keypair

WS_ENDPOINT = "wss://asset-hub-paseo.dotters.network"

SENDER_MNEMONIC = os.environ.get("SENDER_MNEMONIC")
DEST_ADDRESS = os.environ.get(
    "DEST_ADDRESS", "5GgbDVeKZwCmMHzn58iFSgSZDTojRMM52arXnuNXto28R7mg"
)

if not SENDER_MNEMONIC:
    print("SENDER_MNEMONIC not set, skipping")
    sys.exit(0)


def main():
    substrate = SubstrateInterface(url=WS_ENDPOINT)
    print("Python Substrate Interface: Connected to Asset Hub Paseo")

    keypair = Keypair.create_from_mnemonic(SENDER_MNEMONIC)
    print(f"Python Substrate Interface: Sender address: {keypair.ss58_address}")

    call = substrate.compose_call(
        call_module="Balances",
        call_function="transfer_keep_alive",
        call_params={
            "dest": DEST_ADDRESS,
            "value": 1000,
        },
    )

    extrinsic = substrate.create_signed_extrinsic(call=call, keypair=keypair)
    receipt = substrate.submit_extrinsic(extrinsic, wait_for_inclusion=True)

    print(f"  Extrinsic hash: {receipt.extrinsic_hash}")
    print(f"  Block hash: {receipt.block_hash}")
    print(f"  Is success: {receipt.is_success}")

    assert receipt.extrinsic_hash is not None
    assert receipt.block_hash is not None

    substrate.close()
    print("All assertions passed!")


if __name__ == "__main__":
    main()
