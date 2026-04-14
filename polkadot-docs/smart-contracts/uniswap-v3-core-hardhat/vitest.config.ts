import { defineConfig } from "vitest/config";
import { sharedVitestConfig } from "../../../shared/vitest.shared";

export default defineConfig({
  test: {
    ...sharedVitestConfig,
    // Generous timeouts: compile + 187 Hardhat tests + deployment
    testTimeout: 360000, // 6 minutes per test
    hookTimeout: 60000,
    setupFiles: ["./tests/setup.ts"],
    include: ["tests/docs.test.ts"],
  },
});
