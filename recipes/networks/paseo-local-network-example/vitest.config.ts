import { defineConfig } from "vitest/config";
import { sharedVitestConfig } from "../../../shared/vitest.shared";

export default defineConfig({
  test: {
    ...sharedVitestConfig,
    testTimeout: 600000, // 10 minutes
    hookTimeout: 300000, // 5 minutes
    include: ["tests/recipe.test.ts"],
  },
});
