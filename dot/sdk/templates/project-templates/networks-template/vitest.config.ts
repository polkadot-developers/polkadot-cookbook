import { defineConfig } from "vitest/config";

export default defineConfig({
  test: {
    globals: true,
    environment: "node",
    // Long timeouts for network spawn and block production
    testTimeout: 180000,
    hookTimeout: 300000,
    // Run tests sequentially (network spawn depends on prior steps)
    fileParallelism: false,
    sequence: {
      shuffle: false,
    },
    pool: "forks",
    poolOptions: {
      forks: {
        singleFork: true,
      },
    },
    reporters: ["verbose"],
    include: ["tests/network.test.ts"],
  },
});
