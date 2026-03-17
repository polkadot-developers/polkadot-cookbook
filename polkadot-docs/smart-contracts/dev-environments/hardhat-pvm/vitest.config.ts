import { defineConfig } from "vitest/config";

export default defineConfig({
  test: {
    // Run test phases sequentially — each describe block depends on the previous
    fileParallelism: false,
    sequence: {
      shuffle: false,
    },
    // Generous timeouts: PVM compilation and testnet deployments can be slow
    testTimeout: 360000, // 6 minutes per test
    hookTimeout: 60000,
    // Verbose output so CI logs show every assertion
    reporters: ["verbose"],
    // Single fork preserves in-process state across describe blocks
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
