import { config } from "dotenv";
import { join } from "path";

// Explicitly load .env from the project root (the hardhat-pvm/ wrapper directory).
// Vitest's built-in env loading does not reliably propagate to forked child processes,
// so we load it here to ensure PRIVATE_KEY is always available in process.env —
// both locally (via .env file) and in CI (via injected env vars).
config({ path: join(process.cwd(), ".env") });
