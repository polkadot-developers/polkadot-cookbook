import { defineConfig } from "vitest/config";

export default defineConfig({
  test: {
    fileParallelism: false,
    sequence: {
      shuffle: false,
    },
    testTimeout: 300000, // 5 minutes
    hookTimeout: 300000, // 5 minutes for Chopsticks startup
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
