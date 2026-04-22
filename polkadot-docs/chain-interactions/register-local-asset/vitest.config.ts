import { defineConfig } from "vitest/config";
import { sharedVitestConfig } from "../../../shared/vitest.shared";

export default defineConfig({
  test: {
    ...sharedVitestConfig,
    testTimeout: 120000,
    hookTimeout: 180000,
    include: ["tests/docs.test.ts"],
  },
});
