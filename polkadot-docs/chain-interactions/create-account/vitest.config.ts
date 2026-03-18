import { defineConfig } from "vitest/config";
import { sharedVitestConfig } from "../../../shared/vitest.shared";

export default defineConfig({
  test: {
    ...sharedVitestConfig,
    testTimeout: 30000,
    hookTimeout: 10000,
    include: ["tests/docs.test.ts"],
  },
});
