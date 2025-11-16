import { createClient, type TypedApi } from 'polkadot-api';
import { getWsProvider } from 'polkadot-api/ws-provider/node';

/**
 * Helper functions for XCM operations using Polkadot API (PAPI)
 */

/**
 * Create a client connection to a chain
 */
export async function createChainClient(endpoint: string) {
  const provider = getWsProvider(endpoint);
  const client = createClient(provider);
  return client;
}

/**
 * Get the account balance on a specific chain
 */
export async function getBalance(
  api: TypedApi<any>,
  address: string
): Promise<bigint> {
  const accountInfo = await api.query.System.Account.getValue(address);
  return accountInfo.data.free;
}

/**
 * Send a teleport assets transaction from relay chain to parachain
 * Based on best practices from Polkadot docs
 */
export async function teleportAssets(
  api: TypedApi<any>,
  destination: any,
  beneficiary: any,
  assets: any
) {
  const tx = api.tx.XcmPallet.limited_teleport_assets({
    dest: destination,
    beneficiary,
    assets,
    fee_asset_item: 0,
    weight_limit: { Unlimited: undefined },
  });

  return tx;
}

/**
 * Create a destination multiLocation for a parachain
 */
export function createParachainDestination(paraId: number) {
  return {
    V3: {
      parents: 0,
      interior: {
        X1: {
          Parachain: paraId,
        },
      },
    },
  };
}

/**
 * Create a beneficiary multiLocation for an account
 */
export function createAccountBeneficiary(accountId: string) {
  return {
    V3: {
      parents: 0,
      interior: {
        X1: {
          AccountId32: {
            network: undefined,
            id: accountId,
          },
        },
      },
    },
  };
}

/**
 * Create a fungible asset definition
 */
export function createFungibleAsset(amount: bigint) {
  return {
    V3: [
      {
        id: {
          Concrete: {
            parents: 0,
            interior: 'Here',
          },
        },
        fun: {
          Fungible: amount,
        },
      },
    ],
  };
}

/**
 * Wait for a transaction to be finalized
 */
export async function waitForFinalization(txObservable: any): Promise<boolean> {
  return new Promise((resolve, reject) => {
    txObservable.subscribe({
      next: (event: any) => {
        if (event.type === 'finalized') {
          resolve(true);
        }
      },
      error: (error: any) => {
        reject(error);
      },
    });
  });
}
