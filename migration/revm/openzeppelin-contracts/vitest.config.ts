import { defineConfig } from "vitest/config";

export default defineConfig({
  test: {
    // Run tests sequentially
    fileParallelism: false,
    sequence: {
      shuffle: false,
    },
    // Long timeouts for clone and test operations
    testTimeout: 1800000, // 30 minutes
    hookTimeout: 300000, // 5 minutes
    // Show verbose output
    reporters: ["verbose"],
    // Don't run tests in parallel
    pool: "forks",
    poolOptions: {
      forks: {
        singleFork: true,
      },
    },
    // Single test file ensures sequential execution
    include: ["tests/guide.test.ts"],
  },
});
