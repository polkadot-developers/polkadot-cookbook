import type { UserConfig } from "vitest/config";

export const sharedVitestConfig: UserConfig["test"] = {
  fileParallelism: false,
  sequence: { shuffle: false },
  reporters: ["verbose"],
  pool: "forks",
  poolOptions: { forks: { singleFork: true } },
};
