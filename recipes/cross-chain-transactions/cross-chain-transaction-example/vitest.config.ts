import { defineConfig } from "vitest/config";
import { sharedVitestConfig } from "../../../shared/vitest.shared";

export default defineConfig({
  test: {
    ...sharedVitestConfig,
    testTimeout: 300000, // 5 minutes
    hookTimeout: 300000, // 5 minutes for Chopsticks startup
    include: ["tests/recipe.test.ts"],
  },
});
