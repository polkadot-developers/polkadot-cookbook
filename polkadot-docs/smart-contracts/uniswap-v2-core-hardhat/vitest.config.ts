import { defineConfig } from "vitest/config";

export default defineConfig({
  test: {
    // Run tests sequentially — each describe block depends on the previous
    fileParallelism: false,
    sequence: {
      shuffle: false,
    },
    // Generous timeouts: compile + 28 Hardhat tests + deployment
    testTimeout: 360000, // 6 minutes per test
    hookTimeout: 60000,
    // Verbose output
    reporters: ["verbose"],
    // Single fork to preserve in-process state across describe blocks
    pool: "forks",
    poolOptions: {
      forks: {
        singleFork: true,
      },
    },
    setupFiles: ["./tests/setup.ts"],
    include: ["tests/recipe.test.ts"],
  },
});
