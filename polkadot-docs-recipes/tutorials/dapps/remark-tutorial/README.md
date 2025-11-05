# PAPI Account Watcher

## Introduction

This tutorial demonstrates how to build a simple command-line interface (CLI) application that monitors a user's account on the relay chain for the `system.remarkWithEvent` extrinsic, using the Polkadot API (PAPI).

The `system.remarkWithEvent` extrinsic enables the submission of arbitrary data on-chain. In this tutorial, the data consists of a hash derived from the combination of an account address and the word "email" (`address+email`). This hash is monitored on-chain, and the application listens for remarks addressed to the specified account. The `system.remarkWithEvent` extrinsic emits an event that can be observed using the Polkadot API (PAPI).

When the application detects a remark addressed to the specified account, it plays the "You've Got Mail!" sound byte.

## Prerequisites

Before starting, ensure the following tools and dependencies are installed:

- Node.js (version 18 or higher)
- A package manager (npm or yarn)
- Polkadot.js browser extension (wallet)
- An account with Westend tokens

## Setup

Install the required dependencies:

```bash
npm install
```

## Explore the Light Client

The `withLightClient` function creates a light client that synchronizes and interacts with Polkadot directly within the application:

```typescript
async function withLightClient(): Promise<PolkadotClient> {
  // Start the light client
  const smoldot = start();
  // The Westend Relay Chain
  const relayChain = await smoldot.addChain({ chainSpec: westEndChainSpec });
  return createClient(getSmProvider(relayChain));
}
```

The light client functionality is powered by `smoldot`.

## Create the CLI

The CLI includes an option (`-a` / `--account`) to specify the account to monitor for remarks:

```typescript
const program = new Command();
console.log(chalk.white.dim(figlet.textSync('Web3 Mail Watcher')));
program
  .version('0.0.1')
  .description(
    'Web3 Mail Watcher - A simple CLI tool to watch for remarks on the Polkadot network'
  )
  .option('-a, --account <account>', 'Account to watch')
  .parse(process.argv);

const options = program.opts();
```

## Watch for Remarks

The application monitors the Westend network for remarks sent to the specified account:

```typescript
if (options.account) {
  console.log(
    chalk.black.bgRed('Watching account:'),
    chalk.bold.whiteBright(options.account)
  );
  // Create a light client to connect to the Polkadot (Westend) network
  const lightClient = await withLightClient();
  // Get the typed API to interact with the network
  const dotApi = lightClient.getTypedApi(wnd);
  // Subscribe to the System.Remarked event and watch for remarks from the account
  dotApi.event.System.Remarked.watch().subscribe((event) => {
    const { sender, hash } = event.payload;
    const calculatedHash = bytesToHex(
      blake2b(`${options.account}+email`, { dkLen: 32 })
    );
    if (`0x${calculatedHash}` === hash.asHex()) {
      sound.play('youve-got-mail-sound.mp3');
      console.log(chalk.black.bgRed('You got mail!'));
      console.log(
        chalk.black.bgCyan('From:'),
        chalk.bold.whiteBright(sender.toString())
      );
      console.log(
        chalk.black.bgBlue('Hash:'),
        chalk.bold.whiteBright(hash.asHex())
      );
    }
  });
} else {
  console.error('Account is required');
  return;
}
```

## Run

Compile and execute the application:

```bash
npm start -- --account <account-address>
```

For example:

```bash
npm start -- --account 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
```

## Test the CLI

To test the application:

1. Navigate to the **Extrinsics** page of the PAPI Dev Console
2. Select the **System** pallet and the **remark_with_event** call
3. Ensure the input field follows the convention `address+email`
4. Submit the extrinsic and sign it using the Polkadot.js browser wallet

The CLI will display output and play the "You've Got Mail!" sound.

## Next Steps

This application demonstrates how the Polkadot API can be used to build decentralized applications. While this is not a production-grade application, it introduces several key features for developing with the Polkadot API.

To explore more, refer to the [official PAPI documentation](https://papi.how).
