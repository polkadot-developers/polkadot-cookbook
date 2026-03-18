import { defineConfig } from "vitest/config";
import { sharedVitestConfig } from "../../../shared/vitest.shared";

export default defineConfig({
  test: {
    ...sharedVitestConfig,
    testTimeout: 60000,
    hookTimeout: 30000,
    include: ["tests/docs.test.ts"],
  },
});
