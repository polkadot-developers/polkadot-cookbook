import { defineConfig } from "vitest/config";

export default defineConfig({
  test: {
    // Run tests sequentially
    fileParallelism: false,
    sequence: {
      shuffle: false,
    },
    // Long timeouts for clone operations
    testTimeout: 300000, // 5 minutes
    hookTimeout: 60000,
    // Show verbose output
    reporters: ["verbose"],
    // Don't run tests in parallel
    pool: "forks",
    poolOptions: {
      forks: {
        singleFork: true,
      },
    },
    include: ["tests/guide.test.ts"],
  },
});
