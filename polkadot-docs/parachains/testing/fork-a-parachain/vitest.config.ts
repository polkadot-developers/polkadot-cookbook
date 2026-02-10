import { defineConfig } from "vitest/config";

export default defineConfig({
  test: {
    // Run tests sequentially
    fileParallelism: false,
    sequence: {
      shuffle: false,
    },
    // Chopsticks needs time to connect to live chain and fetch state
    testTimeout: 300000, // 5 minutes
    hookTimeout: 120000, // 2 minutes for cleanup
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
