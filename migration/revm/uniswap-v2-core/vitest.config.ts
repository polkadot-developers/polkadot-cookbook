import { defineConfig } from "vitest/config";
import { sharedVitestConfig } from "../../../shared/vitest.shared";

export default defineConfig({
  test: {
    ...sharedVitestConfig,
    testTimeout: 1800000, // 30 minutes
    hookTimeout: 300000, // 5 minutes
    include: ["tests/migration.test.ts"],
  },
});
