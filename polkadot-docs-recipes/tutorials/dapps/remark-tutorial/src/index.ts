import { start } from 'smoldot';
import { createClient, PolkadotClient } from 'polkadot-api';
import { getSmProvider } from '@polkadot-api/sm-provider';
import { wnd } from '@polkadot-api/descriptors';
import { Command } from 'commander';
import chalk from 'chalk';
import figlet from 'figlet';
import { blake2b } from '@noble/hashes/blake2b';
import { bytesToHex } from '@noble/hashes/utils';
import sound from 'play-sound';

// Westend chain spec would need to be imported or fetched
// This is a placeholder - actual implementation needs the full chain spec
const westEndChainSpec = ''; // TODO: Add Westend chain spec

async function withLightClient(): Promise<PolkadotClient> {
  // Start the light client
  const smoldot = start();
  // The Westend Relay Chain
  const relayChain = await smoldot.addChain({ chainSpec: westEndChainSpec });
  return createClient(getSmProvider(relayChain));
}

async function main() {
  const program = new Command();
  console.log(chalk.white.dim(figlet.textSync('Web3 Mail Watcher')));
  program
    .version('0.0.1')
    .description(
      'Web3 Mail Watcher - A simple CLI tool to watch for remarks on the Polkadot network'
    )
    .option('-a, --account <account>', 'Account to watch')
    .parse(process.argv);

  // CLI arguments from commander
  const options = program.opts();

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
}

main();
