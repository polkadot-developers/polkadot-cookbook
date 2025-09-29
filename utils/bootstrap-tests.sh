#!/usr/bin/env bash
set -euo pipefail

ROOT=$(cd "$(dirname "$0")/.." && pwd)
cd "$ROOT"

if [ $# -lt 1 ]; then
  echo "Usage: $0 <tutorial-slug>"
  exit 1
fi

SLUG="$1"
TDIR="tutorials/$SLUG"

if [ ! -d "$TDIR" ]; then
  echo "Tutorial not found: $TDIR"
  exit 1
fi

pushd "$TDIR" >/dev/null
if [ ! -f package.json ]; then
  npm init -y >/dev/null 2>&1 || true
  npm pkg set name=$SLUG type=module >/dev/null 2>&1 || true
fi

npm i -D vitest typescript ts-node @types/node >/dev/null 2>&1 || true
npm i @polkadot/api ws >/dev/null 2>&1 || true

npm pkg set scripts.test="vitest run" scripts.test:watch="vitest" >/dev/null 2>&1 || true

cat > vitest.config.ts << 'EOF'
import { defineConfig } from 'vitest/config';
export default defineConfig({
  test: {
    include: ['tests/**/*.test.ts'],
    testTimeout: 30000,
    hookTimeout: 30000,
  },
});
EOF

cat > tsconfig.json << 'EOF'
{
  "compilerOptions": {
    "target": "ES2020",
    "module": "ESNext",
    "moduleResolution": "Bundler",
    "types": ["node", "vitest/globals"],
    "esModuleInterop": true,
    "resolveJsonModule": true,
    "skipLibCheck": true
  },
  "include": ["tests/**/*.ts"]
}
EOF

echo "Bootstrapped tests for $SLUG"
popd >/dev/null


