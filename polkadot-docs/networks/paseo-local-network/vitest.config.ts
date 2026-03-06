import { defineConfig } from "vitest/config";
import { loadVariables } from "../../shared/load-variables";

const vars = loadVariables();

export default defineConfig({
  test: {
    env: {
      POLKADOT_SDK_VERSION: vars.POLKADOT_SDK_VERSION,
      CHAIN_SPEC_BUILDER_VERSION: vars.CHAIN_SPEC_BUILDER_VERSION,
      PASEO_RUNTIME_VERSION: vars.PASEO_RUNTIME_VERSION,
    },
    // Run test FILES sequentially (critical - setup depends on downloads)
    fileParallelism: false,
    // Run tests within files sequentially
    sequence: {
      shuffle: false,
    },
    // Long timeouts for downloads and network spawn
    testTimeout: 600000, // 10 minutes
    hookTimeout: 300000, // 5 minutes
    // Show verbose output
    reporters: ["verbose"],
    // Don't run tests in parallel
    pool: "forks",
    poolOptions: {
      forks: {
        singleFork: true,
      },
    },
    // Single test file ensures sequential execution
    include: ["tests/guide.test.ts"],
  },
});
