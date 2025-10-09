#!/usr/bin/env node

import fs from 'fs';
import path from 'path';
import { execSync } from 'child_process';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const ROOT = path.resolve(__dirname, '..');

// ANSI colors (no additional dependencies)
const colors = {
  reset: '\x1b[0m',
  red: '\x1b[31m',
  green: '\x1b[32m',
  yellow: '\x1b[33m',
  blue: '\x1b[34m',
  cyan: '\x1b[36m',
};

function log(message, color = 'reset') {
  console.log(`${colors[color]}${message}${colors.reset}`);
}

function error(message) {
  log(`❌ ${message}`, 'red');
}

function success(message) {
  log(`✅ ${message}`, 'green');
}

function info(message) {
  log(`ℹ️  ${message}`, 'cyan');
}

function warning(message) {
  log(`⚠️  ${message}`, 'yellow');
}

// Validate slug format
function isValidSlug(slug) {
  // Must be lowercase, letters, numbers, and dashes only
  const slugRegex = /^[a-z0-9]+(-[a-z0-9]+)*$/;
  return slugRegex.test(slug);
}

// Validate that script is run from repository root
function validateWorkingDirectory() {
  const cwd = process.cwd();

  // Check if tutorials/ folder exists
  if (!fs.existsSync(path.join(cwd, 'tutorials'))) {
    error('This script must be run from the repository root!');
    info('Expected directory structure: ./tutorials/, ./utils/, etc.');
    process.exit(1);
  }

  // Check if versions.yml exists
  if (!fs.existsSync(path.join(cwd, 'versions.yml'))) {
    error('versions.yml not found. Are you in the correct repository?');
    process.exit(1);
  }
}

// Scaffold tutorial structure (from scaffold-tutorial.sh)
function scaffoldStructure(slug) {
  const tutorialDir = path.join(ROOT, 'tutorials', slug);

  // Create directories
  fs.mkdirSync(path.join(tutorialDir, 'tests'), { recursive: true });
  fs.mkdirSync(path.join(tutorialDir, 'scripts'), { recursive: true });
  fs.mkdirSync(path.join(tutorialDir, `${slug}-code`), { recursive: true });

  // Create justfile
  const justfileContent = `default:
  @just --list

say-hello:
  echo "Hello, world!"
`;
  fs.writeFileSync(path.join(tutorialDir, 'justfile'), justfileContent);

  // Create example test
  const testContent = `import { describe, it, expect } from 'vitest';
import { ApiPromise, WsProvider } from '@polkadot/api';
import net from 'node:net';

async function isPortReachable(host: string, port: number, timeoutMs: number): Promise<boolean> {
  return new Promise((resolve) => {
    const socket = new net.Socket();
    const done = (ok: boolean) => { try { socket.destroy(); } catch {} ; resolve(ok); };
    socket.setTimeout(timeoutMs);
    socket.once('error', () => done(false));
    socket.once('timeout', () => done(false));
    socket.connect(port, host, () => done(true));
  });
}

describe('${slug} e2e', () => {
  it('connects and reads chain info', async () => {
    const endpoint = process.env.POLKADOT_WS || 'ws://127.0.0.1:9944';
    const { hostname, port } = new URL(endpoint.replace('ws://', 'http://'));
    if (!(await isPortReachable(hostname, Number(port || 9944), 1000))) {
      console.log('⏭️  Skipping test - node not available');
      return;
    }

    const api = await ApiPromise.create({ provider: new WsProvider(endpoint, 1) });
    const header = await api.rpc.chain.getHeader();
    expect(header.number.toNumber()).toBeGreaterThanOrEqual(0);
    await api.disconnect();
  });
});
`;
  fs.writeFileSync(path.join(tutorialDir, 'tests', `${slug}-e2e.test.ts`), testContent);

  // Create tutorial.yml
  const tutorialYmlContent = `name: ${slug.split('-').map(word => word.charAt(0).toUpperCase() + word.slice(1)).join(' ')}
slug: ${slug}
category: polkadot-sdk-cookbook
needs_node: true
description: Replace with a short description.
type: sdk # or contracts
`;
  fs.writeFileSync(path.join(tutorialDir, 'tutorial.yml'), tutorialYmlContent);

  // Create README.md
  const readmeContent = `# ${slug}

Describe the goal, prerequisites, and step-by-step instructions for this tutorial.

## Prerequisites

- Rust \`1.86+\` (check with \`rustc --version\`)
- Node.js \`20+\` (check with \`node --version\`)
- Basic knowledge of Polkadot SDK

## Steps

1. **Setup environment**
   \`\`\`bash
   cd tutorials/${slug}
   npm install
   \`\`\`

2. **Build the project**
   \`\`\`bash
   # Add your build commands here
   \`\`\`

3. **Run tests**
   \`\`\`bash
   npm run test
   \`\`\`

## Testing

To run the end-to-end tests:

\`\`\`bash
cd tutorials/${slug}
npm run test
\`\`\`

## Next Steps

- Add your implementation code to \`${slug}-code/\`
- Write comprehensive tests in \`tests/\`
- Update this README with detailed instructions
`;
  fs.writeFileSync(path.join(tutorialDir, 'README.md'), readmeContent);

  // Create .gitkeep in scripts/ so folder is committed
  fs.writeFileSync(path.join(tutorialDir, 'scripts', '.gitkeep'), '');
}

// Bootstrap test setup (from bootstrap-tests.sh)
function bootstrapTests(slug) {
  const tutorialDir = path.join(ROOT, 'tutorials', slug);

  // Check if tutorial folder exists
  if (!fs.existsSync(tutorialDir)) {
    error(`Tutorial directory not found: ${tutorialDir}`);
    process.exit(1);
  }

  // Create package.json if it doesn't exist
  const packageJsonPath = path.join(tutorialDir, 'package.json');
  if (!fs.existsSync(packageJsonPath)) {
    execSync(`cd ${tutorialDir} && npm init -y`, { stdio: 'ignore' });
    execSync(`cd ${tutorialDir} && npm pkg set name=${slug} type=module`, { stdio: 'ignore' });
  }

  // Install dev dependencies
  info('Installing dev dependencies (vitest, typescript, ts-node, @types/node)...');
  execSync(`cd ${tutorialDir} && npm i -D vitest typescript ts-node @types/node`, { stdio: 'inherit' });

  // Install dependencies
  info('Installing dependencies (@polkadot/api, ws)...');
  execSync(`cd ${tutorialDir} && npm i @polkadot/api ws`, { stdio: 'inherit' });

  // Set npm scripts
  execSync(`cd ${tutorialDir} && npm pkg set scripts.test="vitest run" scripts.test:watch="vitest" scripts.preview="node ../../common-preview-server/server.js ."`, { stdio: 'ignore' });

  // Create vitest.config.ts
  const vitestConfig = `import { defineConfig } from 'vitest/config';
export default defineConfig({
  test: {
    include: ['tests/**/*.test.ts'],
    testTimeout: 30000,
    hookTimeout: 30000,
  },
});
`;
  fs.writeFileSync(path.join(tutorialDir, 'vitest.config.ts'), vitestConfig);

  // Create tsconfig.json
  const tsconfigContent = `{
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
`;
  fs.writeFileSync(path.join(tutorialDir, 'tsconfig.json'), tsconfigContent);
}

// Main function
async function createTutorial(slug) {
  log('\n🚀 Polkadot Cookbook - Tutorial Creator\n', 'blue');

  // Validate working directory
  validateWorkingDirectory();

  // Validate slug format
  if (!isValidSlug(slug)) {
    error('Invalid tutorial slug format!');
    info('Slug must be lowercase, with words separated by dashes.');
    info('Examples: "my-tutorial", "add-nft-pallet", "zero-to-hero"');
    process.exit(1);
  }

  // Check if tutorial already exists
  const tutorialDir = path.join(ROOT, 'tutorials', slug);
  if (fs.existsSync(tutorialDir)) {
    error(`Tutorial "${slug}" already exists!`);
    info(`Directory: ${tutorialDir}`);
    process.exit(1);
  }

  log(`Creating tutorial: ${slug}\n`, 'cyan');

  // Step 1: Create git branch
  try {
    info('Step 1/4: Creating git branch...');
    const branchName = `feat/tutorial-${slug}`;
    execSync(`git checkout -b ${branchName}`, { stdio: 'inherit' });
    success(`Created branch: ${branchName}`);
  } catch (err) {
    error('Failed to create git branch');
    warning('You may already be on a feature branch. Continue anyway.');
  }

  // Step 2: Scaffold structure
  try {
    info('\nStep 2/4: Scaffolding tutorial structure...');
    scaffoldStructure(slug);
    success('Scaffolded folder structure');
    info(`  - tutorials/${slug}/README.md`);
    info(`  - tutorials/${slug}/tutorial.yml`);
    info(`  - tutorials/${slug}/tests/${slug}-e2e.test.ts`);
    info(`  - tutorials/${slug}/${slug}-code/`);
  } catch (err) {
    error('Failed to scaffold structure');
    console.error(err);
    process.exit(1);
  }

  // Step 3: Bootstrap test environment
  try {
    info('\nStep 3/4: Bootstrapping test environment...');
    bootstrapTests(slug);
    success('Test environment ready');
    info('  - package.json created');
    info('  - vitest, typescript, @polkadot/api installed');
    info('  - vitest.config.ts & tsconfig.json configured');
  } catch (err) {
    error('Failed to bootstrap tests');
    console.error(err);
    process.exit(1);
  }

  // Step 4: Verify setup
  try {
    info('\nStep 4/4: Verifying setup...');
    const packageJsonPath = path.join(tutorialDir, 'package.json');
    const readmePath = path.join(tutorialDir, 'README.md');

    if (fs.existsSync(packageJsonPath) && fs.existsSync(readmePath)) {
      success('All files created successfully!');
    } else {
      warning('Some files may be missing. Please check the tutorial directory.');
    }
  } catch (err) {
    warning('Verification step encountered issues');
  }

  // Success message with next steps
  log('\n' + '='.repeat(60), 'green');
  log('🎉 Tutorial created successfully!', 'green');
  log('='.repeat(60) + '\n', 'green');

  log('📝 Next Steps:', 'yellow');
  console.log('');
  log(`  1. Preview your tutorial live (recommended):`, 'cyan');
  console.log(`     ${colors.reset}cd tutorials/${slug} && npm run preview`);
  console.log('');
  log(`  2. Write your tutorial content:`, 'cyan');
  console.log(`     ${colors.reset}tutorials/${slug}/README.md`);
  console.log('');
  log(`  3. Add your code implementation:`, 'cyan');
  console.log(`     ${colors.reset}tutorials/${slug}/${slug}-code/`);
  console.log('');
  log(`  4. Write comprehensive tests:`, 'cyan');
  console.log(`     ${colors.reset}tutorials/${slug}/tests/`);
  console.log('');
  log(`  5. Run tests to verify:`, 'cyan');
  console.log(`     ${colors.reset}cd tutorials/${slug} && npm test`);
  console.log('');
  log(`  6. Update tutorial.yml metadata:`, 'cyan');
  console.log(`     ${colors.reset}tutorials/${slug}/tutorial.yml`);
  console.log('');
  log(`  7. When ready, open a Pull Request:`, 'cyan');
  console.log(`     ${colors.reset}git add -A`);
  console.log(`     ${colors.reset}git commit -m "feat(tutorial): add ${slug}"`);
  console.log(`     ${colors.reset}git push origin feat/tutorial-${slug}`);
  console.log('');

  log('📚 Need help? Check CONTRIBUTING.md or open an issue!\n', 'blue');
}

// CLI argument parsing
const args = process.argv.slice(2);

if (args.length === 0 || args[0] === '--help' || args[0] === '-h') {
  console.log(`
Usage: npm run create-tutorial <tutorial-slug>

Create a new Polkadot Cookbook tutorial with all necessary scaffolding.

Arguments:
  tutorial-slug    The slug for your tutorial (e.g., "my-tutorial")
                   Must be lowercase with words separated by dashes

Examples:
  npm run create-tutorial zero-to-hero
  npm run create-tutorial add-nft-pallet
  npm run create-tutorial custom-runtime

For more information, see CONTRIBUTING.md
  `);
  process.exit(0);
}

const slug = args[0];
createTutorial(slug).catch((err) => {
  error('An unexpected error occurred:');
  console.error(err);
  process.exit(1);
});
