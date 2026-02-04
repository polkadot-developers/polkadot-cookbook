import { defineConfig } from "vitest/config";

export default defineConfig({
  test: {
    // Run test FILES sequentially (critical - build depends on clone, spawn depends on build)
    fileParallelism: false,
    // Run tests within files sequentially
    sequence: {
      shuffle: false,
    },
    // Long timeouts for build and network spawn
    testTimeout: 1800000, // 30 minutes for builds
    hookTimeout: 300000, // 5 minutes for hooks
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
