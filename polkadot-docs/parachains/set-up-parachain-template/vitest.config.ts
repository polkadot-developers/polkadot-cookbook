import { defineConfig } from "vitest/config";
import { sharedVitestConfig } from "../../../shared/vitest.shared";
import { loadVariables } from "../../shared/load-variables";

const vars = loadVariables();

export default defineConfig({
  test: {
    ...sharedVitestConfig,
    env: {
      POLKADOT_OMNI_NODE_VERSION: vars.POLKADOT_OMNI_NODE_VERSION,
      CHAIN_SPEC_BUILDER_VERSION: vars.CHAIN_SPEC_BUILDER_VERSION,
    },
    testTimeout: 1800000,
    hookTimeout: 300000,
    include: ["tests/docs.test.ts"],
  },
});
