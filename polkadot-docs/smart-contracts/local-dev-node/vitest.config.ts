import { defineConfig } from "vitest/config";

export default defineConfig({
  test: {
    // Phases must run sequentially: clone → build → run node → run eth-rpc → verify
    fileParallelism: false,
    sequence: {
      shuffle: false,
    },
    // Cargo build phases can take up to 60 minutes on a cold CI runner.
    // Individual tests override this with their own second argument where needed.
    testTimeout: 3600000, // 60 minutes
    hookTimeout: 120000,  // 2 minutes for afterAll cleanup
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
