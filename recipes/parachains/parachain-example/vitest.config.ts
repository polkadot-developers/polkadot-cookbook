import { defineConfig } from "vitest/config";
import { sharedVitestConfig } from "../../../shared/vitest.shared";

export default defineConfig({
  test: {
    ...sharedVitestConfig,
    testTimeout: 2700000, // 45 minutes for Rust builds
    hookTimeout: 300000, // 5 minutes for hooks
    include: ["tests/recipe.test.ts"],
  },
});
