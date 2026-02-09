import { defineConfig } from "vitest/config";

export default defineConfig({
  test: {
    fileParallelism: false,
    sequence: {
      shuffle: false,
    },
    testTimeout: 2700000, // 45 minutes for Rust builds
    hookTimeout: 60000,
    reporters: ["verbose"],
    pool: "forks",
    poolOptions: {
      forks: {
        singleFork: true,
      },
    },
    include: ["tests/recipe.test.ts"],
  },
});
