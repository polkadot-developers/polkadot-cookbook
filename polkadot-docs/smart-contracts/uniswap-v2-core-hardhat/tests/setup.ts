import { config } from "dotenv";
import { join } from "path";

// Explicitly load .env from the project root (the uniswap-v2-core-hardhat/ wrapper directory).
// Vitest's built-in env loading does not reliably propagate to forked child processes,
// so we load it here to ensure TESTNET_URL and TESTNET_PRIVATE_KEY are always available
// in process.env — both locally (via .env file) and in CI (via injected env vars).
config({ path: join(process.cwd(), ".env") });
