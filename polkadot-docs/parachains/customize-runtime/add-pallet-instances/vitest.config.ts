import { defineConfig } from "vitest/config";
import { loadVariables } from "../../../shared/load-variables";

const vars = loadVariables();

export default defineConfig({
  test: {
    env: {
      TEMPLATE_VERSION: vars.TEMPLATE_VERSION,
      POLKADOT_OMNI_NODE_VERSION: vars.POLKADOT_OMNI_NODE_VERSION,
      CHAIN_SPEC_BUILDER_VERSION: vars.CHAIN_SPEC_BUILDER_VERSION,
    },
    // Run tests sequentially
    fileParallelism: false,
    sequence: {
      shuffle: false,
    },
    // Long timeouts for build and runtime tests
    testTimeout: 1800000, // 30 minutes for builds
    hookTimeout: 300000, // 5 minutes for hooks
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
