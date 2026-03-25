import { defineConfig } from "vitest/config";
import { sharedVitestConfig } from "../../../shared/vitest.shared";

export default defineConfig({
  test: {
    ...sharedVitestConfig,
    testTimeout: 3600000,
    hookTimeout: 120000,
    include: ["tests/docs.test.ts"],
  },
});
