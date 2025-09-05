#!/bin/bash
set -e
polkadot-omni-node --chain ./chain_spec.json --dev --rpc-cors all --rpc-methods unsafe
