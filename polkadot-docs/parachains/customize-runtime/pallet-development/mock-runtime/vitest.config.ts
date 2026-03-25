import { defineConfig } from "vitest/config";
import { sharedVitestConfig } from "../../../../../shared/vitest.shared";
import { loadVariables } from "../../../../shared/load-variables";

const vars = loadVariables();

export default defineConfig({
  test: {
    ...sharedVitestConfig,
    env: {
      TEMPLATE_VERSION: vars.TEMPLATE_VERSION,
    },
    testTimeout: 1800000,
    hookTimeout: 300000,
    include: ["tests/docs.test.ts"],
  },
});
