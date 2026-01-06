import { defineConfig } from "vitest/config";

export default defineConfig({
  test: {
    // Run test FILES sequentially (critical - runtime depends on build)
    fileParallelism: false,
    // Run tests within files sequentially
    sequence: {
      shuffle: false,
    },
    // Long timeouts for build and runtime tests
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
    // Ensure test files run in order: environment -> build -> runtime
    include: [
      "tests/environment.test.ts",
      "tests/build.test.ts",
      "tests/runtime.test.ts",
    ],
  },
});
