import { createClient, type TypedApi } from 'polkadot-api';
import { getWsProvider } from 'polkadot-api/ws-provider/node';

/**
 * Helper functions for basic chain interactions using Polkadot API (PAPI)
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
 * Get the current block number
 */
export async function getCurrentBlockNumber(api: TypedApi<any>): Promise<number> {
  const header = await api.query.System.Number.getValue();
  return Number(header);
}

/**
 * Get chain metadata
 */
export async function getChainInfo(api: TypedApi<any>) {
  const version = await api.constants.System.Version();
  return {
    specName: version.spec_name,
    specVersion: version.spec_version,
    implName: version.impl_name,
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
