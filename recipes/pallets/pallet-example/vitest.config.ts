import { defineConfig } from "vitest/config";
import { sharedVitestConfig } from "../../../shared/vitest.shared";

export default defineConfig({
  test: {
    ...sharedVitestConfig,
    testTimeout: 2700000, // 45 minutes for Rust builds
    hookTimeout: 60000,
    include: ["tests/recipe.test.ts"],
  },
});
