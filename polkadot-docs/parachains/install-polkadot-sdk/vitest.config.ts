import { defineConfig } from "vitest/config";
import { sharedVitestConfig } from "../../../shared/vitest.shared";

export default defineConfig({
  test: {
    ...sharedVitestConfig,
    testTimeout: 300000,
    hookTimeout: 60000,
    include: ["tests/docs.test.ts"],
  },
});
