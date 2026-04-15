import { defineConfig } from "vitest/config";
import { sharedVitestConfig } from "../../../shared/vitest.shared";

export default defineConfig({
  test: {
    ...sharedVitestConfig,
    testTimeout: 360000,
    hookTimeout: 120000,
    setupFiles: ["./tests/setup.ts"],
    include: ["tests/docs.test.ts"],
  },
});
