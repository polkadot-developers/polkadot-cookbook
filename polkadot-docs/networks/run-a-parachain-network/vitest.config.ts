import { defineConfig } from "vitest/config";
import { sharedVitestConfig } from "../../../shared/vitest.shared";
import { loadVariables } from "../../shared/load-variables";

const vars = loadVariables();

export default defineConfig({
  test: {
    ...sharedVitestConfig,
    env: {
      TEMPLATE_VERSION: vars.TEMPLATE_VERSION,
      POLKADOT_SDK_VERSION: vars.POLKADOT_SDK_VERSION,
      CHAIN_SPEC_BUILDER_VERSION: vars.CHAIN_SPEC_BUILDER_VERSION,
      PASEO_RUNTIME_VERSION: vars.PASEO_RUNTIME_VERSION,
    },
    testTimeout: 1800000,
    hookTimeout: 300000,
    include: ["tests/docs.test.ts"],
  },
});
